//! this mod implements InputSource on TransactionRecordsById

use orchard::note_encryption::OrchardDomain;
use sapling_crypto::note_encryption::SaplingDomain;
use zcash_client_backend::{
    data_api::{InputSource, SpendableNotes},
    wallet::{ReceivedNote, WalletTransparentOutput},
    ShieldedProtocol,
};
use zcash_primitives::{
    legacy::Script,
    transaction::{
        components::{amount::NonNegativeAmount, TxOut},
        fees::zip317::MARGINAL_FEE,
        TxId,
    },
};

use crate::wallet::{
    notes::{query::OutputSpendStatusQuery, OutputInterface},
    transaction_records_by_id::TransactionRecordsById,
};

// error type
use std::fmt::Debug;
use thiserror::Error;

use zcash_client_backend::wallet::NoteId;
use zcash_primitives::transaction::components::amount::BalanceError;

/// Error type used by InputSource trait
#[derive(Debug, PartialEq, Error)]
pub enum InputSourceError {
    /// No witness position found for note. Note cannot be spent.
    #[error("No witness position found for note. Note cannot be spent: {0:?}")]
    WitnessPositionNotFound(NoteId),
    /// Value outside the valid range of zatoshis
    #[error("Value outside valid range of zatoshis. {0:?}")]
    InvalidValue(BalanceError),
    /// Wallet data is out of date
    #[error("Output index data is missing! Wallet data is out of date, please rescan.")]
    MissingOutputIndexes(Vec<(TxId, zcash_primitives::consensus::BlockHeight)>),
}

// Calculate remaining difference between target and selected.
// There are two mutually exclusive cases:
//    (Change) There's no more needed so we've selected 0 or more change
//    (Positive) We need > 0 more value.
// This function represents the NonPositive case as None, which
// then serves to signal a break in the note selection for where
// this helper is uniquely called.
fn calculate_remaining_needed(
    target_value: NonNegativeAmount,
    selected_value: NonNegativeAmount,
) -> RemainingNeeded {
    if let Some(amount) = target_value - selected_value {
        if amount == NonNegativeAmount::ZERO {
            // Case (Change) target_value == total_selected_value
            RemainingNeeded::GracelessChangeAmount(NonNegativeAmount::ZERO)
        } else {
            // Case (Positive) target_value > total_selected_value
            RemainingNeeded::Positive(amount)
        }
    } else {
        // Case (Change) target_value < total_selected_value
        // Return the non-zero change quantity
        RemainingNeeded::GracelessChangeAmount(
            (selected_value - target_value).expect("This is guaranteed positive"),
        )
    }
}
enum RemainingNeeded {
    Positive(NonNegativeAmount),
    GracelessChangeAmount(NonNegativeAmount),
}

#[allow(dead_code)]
fn sweep_dust_into_grace(
    selected: &mut Vec<(NoteId, NonNegativeAmount)>,
    dust_notes: Vec<(NoteId, NonNegativeAmount)>,
) {
    let sapling_dust: Vec<_> = dust_notes
        .iter()
        .filter(|x| x.0.protocol() == ShieldedProtocol::Sapling)
        .cloned()
        .collect();
    let orchard_dust: Vec<_> = dust_notes
        .iter()
        .filter(|x| x.0.protocol() == ShieldedProtocol::Orchard)
        .cloned()
        .collect();
    let sapling_selected: Vec<_> = selected
        .iter()
        .filter(|x| x.0.protocol() == ShieldedProtocol::Sapling)
        .cloned()
        .collect();
    let orchard_selected: Vec<_> = selected
        .iter()
        .filter(|x| x.0.protocol() == ShieldedProtocol::Orchard)
        .cloned()
        .collect();
    if sapling_selected.len() == 1 && !sapling_dust.is_empty() {
        selected.push(*sapling_dust.last().expect("Guaranteed by !is_empty"));
    }
    if orchard_selected.len() == 1 && !orchard_dust.is_empty() {
        selected.push(*orchard_dust.last().expect("Guaranteed by !is_empty"));
    }
}
/// A trait representing the capability to query a data store for unspent transaction outputs
/// belonging to a wallet.
impl InputSource for TransactionRecordsById {
    /// The type of errors produced by a wallet backend.
    /// IMPL: zingolib's error type. This could
    /// maybe more specific
    type Error = InputSourceError;
    /// Backend-specific account identifier.
    ///
    /// An account identifier corresponds to at most a single unified spending key's worth of spend
    /// authority, such that both received notes and change spendable by that spending authority
    /// will be interpreted as belonging to that account. This might be a database identifier type
    /// or a UUID.
    /// IMPL: We only use this as much as we are
    /// forced to by the interface, zingo does
    /// not support multiple accounts at present
    type AccountId = zcash_primitives::zip32::AccountId;
    /// Backend-specific note identifier.
    ///
    /// For example, this might be a database identifier type or a UUID.
    /// IMPL: We identify notes by
    /// txid, domain, and index
    type NoteRef = NoteId;

    /// not implemented
    fn get_spendable_note(
        &self,
        _txid: &zcash_primitives::transaction::TxId,
        _protocol: zcash_client_backend::ShieldedProtocol,
        _index: u32,
    ) -> Result<
        Option<
            zcash_client_backend::wallet::ReceivedNote<
                Self::NoteRef,
                zcash_client_backend::wallet::Note,
            >,
        >,
        Self::Error,
    > {
        unimplemented!()
    }

    #[allow(rustdoc::private_intra_doc_links)]
    /// the trait method below is used as a TxMapAndMaybeTrees trait method by propose_transaction.
    /// this function is used inside a loop that calculates a fee and balances change
    /// this algorithm influences strategy for user fee minimization
    /// see [crate::lightclient::LightClient::create_send_proposal]
    /// TRAIT DOCUMENTATION
    /// Returns a list of spendable notes sufficient to cover the specified target value, if
    /// possible. Only spendable notes corresponding to the specified shielded protocol will
    /// be included.
    /// IMPLEMENTATION DETAILS
    /// account skipped because Zingo uses 1 account.
    /// the algorithm to pick notes is summarized below
    /// 1) first, sort all eligible notes by value
    /// 2) pick the smallest that overcomes the target value and return it
    /// 3) if you cant, add the biggest to the selection and return to step 2 to pick another
    fn select_spendable_notes(
        &self,
        _account: Self::AccountId,
        target_value: zcash_primitives::transaction::components::amount::NonNegativeAmount,
        sources: &[zcash_client_backend::ShieldedProtocol],
        anchor_height: zcash_primitives::consensus::BlockHeight,
        exclude: &[Self::NoteRef],
    ) -> Result<SpendableNotes<Self::NoteRef>, Self::Error> {
        let mut unselected = self
            .get_spendable_note_ids_and_values(sources, anchor_height, exclude)
            .map_err(InputSourceError::MissingOutputIndexes)?
            .into_iter()
            .map(|(id, value)| NonNegativeAmount::from_u64(value).map(|value| (id, value)))
            .collect::<Result<Vec<_>, _>>()
            .map_err(InputSourceError::InvalidValue)?;
        unselected.sort_by_key(|(_id, value)| *value); // from smallest to largest
        let dust_spendable_index = unselected.partition_point(|(_id, value)| *value < MARGINAL_FEE);
        let _dust_notes: Vec<_> = unselected.drain(..dust_spendable_index).collect();
        let mut selected = vec![];
        let mut index_of_unselected = 0;

        loop {
            // if no unselected notes are available, return the currently selected notes even if the target value has not been reached
            if unselected.is_empty() {
                break;
            }
            // update target value for further note selection
            let selected_notes_total_value = selected
                .iter()
                .try_fold(NonNegativeAmount::ZERO, |acc, (_id, value)| acc + *value)
                .ok_or(InputSourceError::InvalidValue(BalanceError::Overflow))?;
            let updated_target_value =
                match calculate_remaining_needed(target_value, selected_notes_total_value) {
                    RemainingNeeded::Positive(updated_target_value) => updated_target_value,
                    RemainingNeeded::GracelessChangeAmount(_change) => {
                        //println!("{:?}", change);
                        break;
                    }
                };

            match unselected.get(index_of_unselected) {
                Some(smallest_unselected) => {
                    // selected a note to test if it has enough value to complete the transaction on its own
                    if smallest_unselected.1 >= updated_target_value {
                        selected.push(*smallest_unselected);
                        unselected.remove(index_of_unselected);
                    } else {
                        // this note is not big enough. try the next
                        index_of_unselected += 1;
                    }
                }
                None => {
                    // the iterator went off the end of the vector without finding a note big enough to complete the transaction
                    // add the biggest note and reset the iteraton
                    selected.push(unselected.pop().expect("should be nonempty")); // TODO:  Add soundness proving unit-test
                    index_of_unselected = 0;
                }
            }
        }

        /* TODO: Priority
        if selected
            .iter()
            .filter(|n| n.0.protocol() == ShieldedProtocol::Sapling)
            .count()
            == 1
            || selected
                .iter()
                .filter(|n| n.0.protocol() == ShieldedProtocol::Orchard)
                .count()
                == 1
        {
            // since we maxed out the target value with only one note in at least one Shielded Pool
            //  we have an option to sweep a dust note into a grace input.
            // we will sweep the biggest dust note we can
            if !dust_notes.is_empty() {
                sweep_dust_into_grace(&mut selected, dust_notes);
            }
            // TODO: re-introduce this optimisation, current bug is that we don't select a note from the same pool as the single selected note
            // (and we don't have information about the pool(s) the outputs are being created for)
            // this is ok for dust as it is excluded if the dust is from a pool where grace inputs are available. however, this doesn't work for
            // non-dust
            //
            // } else {
            //     // we have no extra dust, but we can still save a marginal fee by adding the next smallest note to change
            //     if let Some(smallest_note) = unselected.pop() {
            //         selected.push(smallest_note);
            //     };
            // }
        }
        */

        let mut selected_sapling = Vec::<ReceivedNote<NoteId, sapling_crypto::Note>>::new();
        let mut selected_orchard = Vec::<ReceivedNote<NoteId, orchard::Note>>::new();

        // transform each NoteId to a ReceivedNote
        selected.iter().try_for_each(|(id, _value)| {
            let transaction_record = self
                .get(id.txid())
                .expect("should exist as note_id is created from the record itself");
            let output_index = id.output_index() as u32;
            match id.protocol() {
                zcash_client_backend::ShieldedProtocol::Sapling => transaction_record
                    .get_received_note::<SaplingDomain>(output_index)
                    .map(|received_note| {
                        selected_sapling.push(received_note);
                    }),
                zcash_client_backend::ShieldedProtocol::Orchard => transaction_record
                    .get_received_note::<OrchardDomain>(output_index)
                    .map(|received_note| {
                        selected_orchard.push(received_note);
                    }),
            }
            .ok_or(InputSourceError::WitnessPositionNotFound(*id))
        })?;

        Ok(SpendableNotes::new(selected_sapling, selected_orchard))
    }

    /// Fetches a spendable transparent output.
    ///
    /// Returns `Ok(None)` if the UTXO is not known to belong to the wallet or is not
    /// spendable.
    /// IMPL: Implemented and tested
    fn get_unspent_transparent_output(
        &self,
        outpoint: &zcash_primitives::transaction::components::OutPoint,
    ) -> Result<Option<zcash_client_backend::wallet::WalletTransparentOutput>, Self::Error> {
        let Some((height, output)) = self.values().find_map(|transaction_record| {
            transaction_record
                .transparent_outputs
                .iter()
                .find_map(|output| {
                    if &output.to_outpoint() == outpoint {
                        transaction_record
                            .status
                            .get_confirmed_height()
                            .map(|height| (height, output))
                    } else {
                        None
                    }
                })
                .filter(|(_height, output)| {
                    output.spend_status_query(OutputSpendStatusQuery::only_unspent())
                })
        }) else {
            return Ok(None);
        };
        let value =
            NonNegativeAmount::from_u64(output.value).map_err(InputSourceError::InvalidValue)?;

        let script_pubkey = Script(output.script.clone());

        Ok(WalletTransparentOutput::from_parts(
            outpoint.clone(),
            TxOut {
                value,
                script_pubkey,
            },
            height,
        ))
    }
    /// Returns a list of unspent transparent UTXOs that appear in the chain at heights up to and
    /// including `max_height`.
    /// IMPL: Implemented and tested. address is unused, we select all outputs available to the wallet.
    /// IMPL: _address skipped because Zingo uses 1 account.
    fn get_unspent_transparent_outputs(
        &self,
        // I don't understand what this argument is for. Is the Trait's intent to only shield
        // utxos from one address at a time? Is this needed?
        _address: &zcash_primitives::legacy::TransparentAddress,
        max_height: zcash_primitives::consensus::BlockHeight,
        exclude: &[zcash_primitives::transaction::components::OutPoint],
    ) -> Result<Vec<zcash_client_backend::wallet::WalletTransparentOutput>, Self::Error> {
        self.values()
            .filter_map(|transaction_record| {
                transaction_record
                    .status
                    .get_confirmed_height()
                    .map(|height| (transaction_record, height))
                    .filter(|(_, height)| height <= &max_height)
            })
            .flat_map(|(transaction_record, confirmed_height)| {
                transaction_record
                    .transparent_outputs
                    .iter()
                    .filter(|output| {
                        exclude
                            .iter()
                            .all(|excluded| excluded != &output.to_outpoint())
                    })
                    .filter(|output| {
                        output.spend_status_query(OutputSpendStatusQuery::only_unspent())
                    })
                    .filter_map(move |output| {
                        let value = match NonNegativeAmount::from_u64(output.value)
                            .map_err(InputSourceError::InvalidValue)
                        {
                            Ok(v) => v,
                            Err(e) => return Some(Err(e)),
                        };

                        let script_pubkey = Script(output.script.clone());
                        Ok(WalletTransparentOutput::from_parts(
                            output.to_outpoint(),
                            TxOut {
                                value,
                                script_pubkey,
                            },
                            confirmed_height,
                        ))
                        .transpose()
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prop_assert_eq, proptest};
    use zcash_client_backend::{data_api::InputSource as _, ShieldedProtocol};
    use zcash_primitives::{
        consensus::BlockHeight, legacy::TransparentAddress,
        transaction::components::amount::NonNegativeAmount,
    };
    use zip32::AccountId;

    use crate::wallet::{
        notes::{orchard::mocks::OrchardNoteBuilder, transparent::mocks::TransparentOutputBuilder},
        transaction_record::mocks::{
            nine_note_transaction_record_default, TransactionRecordBuilder,
        },
        transaction_records_by_id::TransactionRecordsById,
    };

    proptest! {
        // TODO: rewrite select_spendable test suite to test a range of cases and target edge cases correctly
        // #[test]
        // fn select_spendable_notes( sapling_value in 5_000..10_000_000u32,
        //     orchard_value in 5_000..10_000_000u32,
        //     target_value in 5_000..10_000_000u32,
        // ) {
        //     let mut transaction_records_by_id = TransactionRecordsById::new();
        //     transaction_records_by_id.insert_transaction_record(nine_note_transaction_record(
        //         1_000_000_u64,
        //         1_000_000_u64,
        //         1_000_000_u64,
        //         sapling_value as u64,
        //         1_000_000_u64,
        //         1_000_000_u64,
        //         orchard_value as u64,
        //         1_000_000_u64,
        //         1_000_000_u64,
        //     ));

        //     let target_amount = NonNegativeAmount::const_from_u64(target_value as u64);
        //     let anchor_height: BlockHeight = 10.into();
        //     let spendable_notes =
        //         zcash_client_backend::data_api::InputSource::select_spendable_notes(
        //             &transaction_records_by_id,
        //             AccountId::ZERO,
        //             target_amount,
        //             &[ShieldedProtocol::Sapling, ShieldedProtocol::Orchard],
        //             anchor_height,
        //             &[],
        //         ).unwrap();
        //     prop_assert_eq!(spendable_notes.sapling().len(), 1);
        //     prop_assert_eq!(spendable_notes.orchard().len(), 1);
        // }
        #[test]
        fn select_spendable_notes_2(
            target_value in 5_000..3_980_000u32,
        ) {
            let mut transaction_records_by_id = TransactionRecordsById::new();
            transaction_records_by_id.insert_transaction_record(

        TransactionRecordBuilder::default()
            .orchard_notes(OrchardNoteBuilder::default().value(1_000_000).clone())
            .orchard_notes(OrchardNoteBuilder::default().value(1_000_000).clone())
            .orchard_notes(OrchardNoteBuilder::default().value(1_000_000).clone())
            .orchard_notes(OrchardNoteBuilder::default().value(1_000_000).clone())
            // .orchard_notes(OrchardNoteBuilder::default().value(0).clone())
            // .orchard_notes(OrchardNoteBuilder::default().value(1).clone())
            // .orchard_notes(OrchardNoteBuilder::default().value(10).clone())
            .randomize_txid()
            .set_output_indexes()
            .build()
                );

            let target_amount = NonNegativeAmount::const_from_u64(target_value as u64);
            let anchor_height: BlockHeight = 10.into();
            let spendable_notes =
                zcash_client_backend::data_api::InputSource::select_spendable_notes(
                    &transaction_records_by_id,
                    AccountId::ZERO,
                    target_amount,
                    &[ShieldedProtocol::Sapling, ShieldedProtocol::Orchard],
                    anchor_height,
                    &[],
                ).unwrap();
            let expected_len = match target_value {
                target_value if target_value <= 1_000_000 => 1,
                target_value if target_value <= 2_000_000 => 2,
                target_value if target_value <= 3_000_000 => 3,
                _ => 4
            };

            prop_assert_eq!(spendable_notes.sapling().len() + spendable_notes.orchard().len(), expected_len);
        }
    }

    #[test]
    fn get_unspent_transparent_output() {
        let mut transaction_records_by_id = TransactionRecordsById::new();

        let transaction_record = nine_note_transaction_record_default();

        transaction_records_by_id.insert_transaction_record(transaction_record);

        let transparent_output = transaction_records_by_id
            .0
            .values()
            .next()
            .unwrap()
            .transparent_outputs
            .first()
            .unwrap();
        let record_height = transaction_records_by_id
            .0
            .values()
            .next()
            .unwrap()
            .status
            .get_confirmed_height();

        let wto = transaction_records_by_id
            .get_unspent_transparent_output(
                &TransparentOutputBuilder::default().build().to_outpoint(),
            )
            .unwrap()
            .unwrap();

        assert_eq!(wto.outpoint(), &transparent_output.to_outpoint());
        assert_eq!(wto.txout().value.into_u64(), transparent_output.value);
        assert_eq!(wto.txout().script_pubkey.0, transparent_output.script);
        assert_eq!(Some(wto.height()), record_height)
    }

    #[test]
    fn get_unspent_transparent_outputs() {
        let mut transaction_records_by_id = TransactionRecordsById::new();
        transaction_records_by_id.insert_transaction_record(nine_note_transaction_record_default());

        let transparent_output = transaction_records_by_id
            .0
            .values()
            .next()
            .unwrap()
            .transparent_outputs
            .first()
            .unwrap();
        let record_height = transaction_records_by_id
            .0
            .values()
            .next()
            .unwrap()
            .status
            .get_confirmed_height();

        let selected_outputs = transaction_records_by_id
            .get_unspent_transparent_outputs(
                &TransparentAddress::ScriptHash([0; 20]),
                BlockHeight::from_u32(10),
                &[],
            )
            .unwrap();
        assert_eq!(selected_outputs.len(), 1);
        assert_eq!(
            selected_outputs.first().unwrap().outpoint(),
            &transparent_output.to_outpoint()
        );
        assert_eq!(
            selected_outputs.first().unwrap().txout().value.into_u64(),
            transparent_output.value
        );
        assert_eq!(
            selected_outputs.first().unwrap().txout().script_pubkey.0,
            transparent_output.script
        );
        assert_eq!(
            Some(selected_outputs.first().unwrap().height()),
            record_height
        )
    }
}
