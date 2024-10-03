use zcash_client_backend::PoolType;
use zcash_client_backend::ShieldedProtocol;

use super::super::LightWallet;

/// ExampleWalletNetworkCase sorts first by Network, then seed, then last saved version.
/// It is public so that any consumer can select and load any example wallet.
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleWalletNetwork {
    /// /
    Regtest(ExampleRegtestWalletSeed),
    /// /
    Testnet(ExampleTestnetWalletSeed),
    /// /
    Mainnet(ExampleMainnetWalletSeed),
}

/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleMainnetWalletSeed {
    /// this is a mainnet wallet originally called missing_data_test
    VTFCORFBCBPCTCFUPMEGMWBP(ExampleVTFCORFBCBPCTCFUPMEGMWBPVersion),
    /// empty mainnet wallet
    HHCCLALTPCCKCSSLPCNETBLR(ExampleHHCCLALTPCCKCSSLPCNETBLRVersion),
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleVTFCORFBCBPCTCFUPMEGMWBPVersion {
    /// wallet was last saved in this serialization version
    V28,
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleHHCCLALTPCCKCSSLPCNETBLRVersion {
    /// wallet was last saved in this serialization version
    Gf0aaf9347,
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleTestnetWalletSeed {
    /// this testnet wallet was generated at the beginning of serialization v28 by fluidvanadium
    CBBHRWIILGBRABABSSHSMTPR(ExampleCBBHRWIILGBRABABSSHSMTPRVersion),
    /// This is a testnet seed, generated by fluidvanadium at the beginning of owls (this example wallet enumeration)
    MSKMGDBHOTBPETCJWCSPGOPP(ExampleMSKMGDBHOTBPETCJWCSPGOPPVersion),
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleMSKMGDBHOTBPETCJWCSPGOPPVersion {
    /// wallet was last saved by the code in this commit
    Gab72a38b,
    /// this wallet was synced in this version. does it have a bunch of taz scattered around different addresses?
    G93738061a,
    /// NU6 added to zingolib re-allows testnet tests by this commit
    Ga74fed621,
}
/// A testnet wallet initiated with
/// --seed "chimney better bulb horror rebuild whisper improve intact letter giraffe brave rib appear bulk aim burst snap salt hill sad merge tennis phrase raise"
/// with 3 addresses containing all receivers.
/// including orchard and sapling transactions
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleCBBHRWIILGBRABABSSHSMTPRVersion {
    /// wallet was last saved in this serialization version
    V26,
    /// wallet was last saved in this serialization version
    V27,
    /// wallet was last saved in this serialization version
    V28,
    /// wallet was last saved at this commit
    G2f3830058,
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleRegtestWalletSeed {
    /// this is a regtest wallet originally called old_wallet_reorg_test_wallet
    HMVASMUVWMSSVICHCARBPOCT(ExampleHMVASMUVWMSSVICHCARBPOCTVersion),
    /// this is a regtest wallet originally called v26/sap_only
    AAAAAAAAAAAAAAAAAAAAAAAA(ExampleAAAAAAAAAAAAAAAAAAAAAAAAVersion),
    /// another regtest wallet
    AADAALACAADAALACAADAALAC(ExampleAADAALACAADAALACAADAALACVersion),
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleHMVASMUVWMSSVICHCARBPOCTVersion {
    /// wallet was last saved in this serialization version
    V27,
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleAAAAAAAAAAAAAAAAAAAAAAAAVersion {
    /// wallet was last saved in this serialization version
    V26,
}
/// /
#[non_exhaustive]
#[derive(Clone)]
pub enum ExampleAADAALACAADAALACAADAALACVersion {
    /// existing receivers?
    OrchAndSapl,
    /// existing receivers?
    OrchOnly,
}

impl ExampleWalletNetwork {
    /// loads test wallets
    /// this function can be improved by a macro. even better would be to derive directly from the enum.
    /// loads any one of the test wallets included in the examples
    pub async fn load_example_wallet(&self) -> LightWallet {
        match self {
            ExampleWalletNetwork::Regtest(example_regt_seed) => match example_regt_seed {
                ExampleRegtestWalletSeed::HMVASMUVWMSSVICHCARBPOCT(
                    ExampleHMVASMUVWMSSVICHCARBPOCTVersion::V27,
                ) => {
                    LightWallet::unsafe_from_buffer_regtest(include_bytes!(
                        "examples/regtest/hmvasmuvwmssvichcarbpoct/v27/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleRegtestWalletSeed::AAAAAAAAAAAAAAAAAAAAAAAA(
                    ExampleAAAAAAAAAAAAAAAAAAAAAAAAVersion::V26,
                ) => {
                    LightWallet::unsafe_from_buffer_regtest(include_bytes!(
                        "examples/regtest/aaaaaaaaaaaaaaaaaaaaaaaa/v26/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleRegtestWalletSeed::AADAALACAADAALACAADAALAC(
                    ExampleAADAALACAADAALACAADAALACVersion::OrchAndSapl,
                ) => {
                    LightWallet::unsafe_from_buffer_regtest(include_bytes!(
                        "examples/regtest/aadaalacaadaalacaadaalac/orch_and_sapl/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleRegtestWalletSeed::AADAALACAADAALACAADAALAC(
                    ExampleAADAALACAADAALACAADAALACVersion::OrchOnly,
                ) => {
                    LightWallet::unsafe_from_buffer_regtest(include_bytes!(
                        "examples/regtest/aadaalacaadaalacaadaalac/orch_only/zingo-wallet.dat"
                    ))
                    .await
                }
            },
            ExampleWalletNetwork::Testnet(example_testnet_seed) => match example_testnet_seed {
                ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    ExampleCBBHRWIILGBRABABSSHSMTPRVersion::V26,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/cbbhrwiilgbrababsshsmtpr/v26/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    ExampleCBBHRWIILGBRABABSSHSMTPRVersion::V27,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/cbbhrwiilgbrababsshsmtpr/v27/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    ExampleCBBHRWIILGBRABABSSHSMTPRVersion::V28,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/cbbhrwiilgbrababsshsmtpr/v28/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    ExampleCBBHRWIILGBRABABSSHSMTPRVersion::G2f3830058,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/cbbhrwiilgbrababsshsmtpr/G2f3830058/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(
                    ExampleMSKMGDBHOTBPETCJWCSPGOPPVersion::Gab72a38b,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/mskmgdbhotbpetcjwcspgopp/Gab72a38b/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(
                    ExampleMSKMGDBHOTBPETCJWCSPGOPPVersion::G93738061a,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/mskmgdbhotbpetcjwcspgopp/G93738061a/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(
                    ExampleMSKMGDBHOTBPETCJWCSPGOPPVersion::Ga74fed621,
                ) => {
                    LightWallet::unsafe_from_buffer_testnet(include_bytes!(
                        "examples/testnet/mskmgdbhotbpetcjwcspgopp/Ga74fed621/zingo-wallet.dat"
                    ))
                    .await
                }
            },
            ExampleWalletNetwork::Mainnet(example_mainnet_seed) => match example_mainnet_seed {
                ExampleMainnetWalletSeed::VTFCORFBCBPCTCFUPMEGMWBP(
                    ExampleVTFCORFBCBPCTCFUPMEGMWBPVersion::V28,
                ) => {
                    LightWallet::unsafe_from_buffer_mainnet(include_bytes!(
                        "examples/mainnet/vtfcorfbcbpctcfupmegmwbp/v28/zingo-wallet.dat"
                    ))
                    .await
                }
                ExampleMainnetWalletSeed::HHCCLALTPCCKCSSLPCNETBLR(
                    ExampleHHCCLALTPCCKCSSLPCNETBLRVersion::Gf0aaf9347,
                ) => {
                    LightWallet::unsafe_from_buffer_mainnet(include_bytes!(
                        "examples/mainnet/hhcclaltpcckcsslpcnetblr/gf0aaf9347/zingo-wallet.dat"
                    ))
                    .await
                }
            },
        }
    }
    /// picks the seed (or ufvk) string associated with an example wallet
    pub fn example_wallet_base(&self) -> String {
        match self {
            ExampleWalletNetwork::Regtest(example_regt_seed) => match example_regt_seed {
                ExampleRegtestWalletSeed::HMVASMUVWMSSVICHCARBPOCT(_) => {
                    crate::testvectors::seeds::HOSPITAL_MUSEUM_SEED.to_string()
                },
                ExampleRegtestWalletSeed::AAAAAAAAAAAAAAAAAAAAAAAA(_) => {
                    crate::testvectors::seeds::ABANDON_ART_SEED.to_string()
                },
                ExampleRegtestWalletSeed::AADAALACAADAALACAADAALAC(_) => {
                    "absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice comic".to_string()
                }
            },
            ExampleWalletNetwork::Testnet(example_testnet_seed) => match example_testnet_seed {
                ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    _,
                ) => crate::testvectors::seeds::CHIMNEY_BETTER_SEED.to_string(),
                ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(
                    _,
                ) => "mobile shuffle keen mother globe desk bless hub oil town begin potato explain table crawl just wild click spring pottery gasp often pill plug".to_string()
            },
            ExampleWalletNetwork::Mainnet(example_mainnet_seed) => match example_mainnet_seed {
                ExampleMainnetWalletSeed::VTFCORFBCBPCTCFUPMEGMWBP(
                     _,
                ) => "village target fun course orange release female brain cruise birth pet copy trouble common fitness unfold panther man enjoy genuine merry write bulb pledge".to_string(),
                ExampleMainnetWalletSeed::HHCCLALTPCCKCSSLPCNETBLR(
                     _,
                ) => "hotel humor crunch crack language awkward lunar term priority critic cushion keep coin sketch soap laugh pretty cement noodle enjoy trip bicycle list return".to_string()
            }
        }
    }
    /// picks the first receiver associated with an example wallet
    pub fn example_wallet_address(&self, pool: PoolType) -> String {
        match pool {
            PoolType::Transparent => match self {
                ExampleWalletNetwork::Regtest(example_regt_seed) => match example_regt_seed {
                    ExampleRegtestWalletSeed::HMVASMUVWMSSVICHCARBPOCT(_) => {"tmFLszfkjgim4zoUMAXpuohnFBAKy99rr2i".to_string()},
                    ExampleRegtestWalletSeed::AAAAAAAAAAAAAAAAAAAAAAAA(_) => {"tmBsTi2xWTjUdEXnuTceL7fecEQKeWaPDJd".to_string()},
                    ExampleRegtestWalletSeed::AADAALACAADAALACAADAALAC(_) => {"tmS9nbexug7uT8x1cMTLP1ABEyKXpMjR5F1".to_string()},
                },
                ExampleWalletNetwork::Testnet(example_test_seed) => match example_test_seed {
                    ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(_) => {"tmYd5GP6JxUxTUcz98NLPumEotvaMPaXytz".to_string()},
                    ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(_) => {"tmEVmDAnveCakZkvV4a6FT1TfYApTv937E7".to_string()},
                },
                ExampleWalletNetwork::Mainnet(example_mainn_seed) => match example_mainn_seed {
                    ExampleMainnetWalletSeed::VTFCORFBCBPCTCFUPMEGMWBP(_) => {"t1P8tQtYFLR7TWsqtauc71RGQdqqwfFBbb4".to_string()},
                    ExampleMainnetWalletSeed::HHCCLALTPCCKCSSLPCNETBLR(_) => {"t1XnsupYhvhSDSFJ4nzZ2kADhLMR22wg35y".to_string()},
                }
            },
            PoolType::Shielded(ShieldedProtocol::Sapling) => match self {
                ExampleWalletNetwork::Regtest(ExampleRegtestWalletSeed::HMVASMUVWMSSVICHCARBPOCT(
                    _,
                )) => "zregtestsapling1fkc26vpg566hgnx33n5uvgye4neuxt4358k68atnx78l5tg2dewdycesmr4m5pn56ffzsa7lyj6".to_string(),
                ExampleWalletNetwork::Regtest(ExampleRegtestWalletSeed::AAAAAAAAAAAAAAAAAAAAAAAA(
                    _,
                )) => "zregtestsapling1fmq2ufux3gm0v8qf7x585wj56le4wjfsqsj27zprjghntrerntggg507hxh2ydcdkn7sx8kya7p".to_string(),
                ExampleWalletNetwork::Regtest(ExampleRegtestWalletSeed::AADAALACAADAALACAADAALAC(
                    _,
                )) => "zregtestsapling1lhjvuj4s3ghhccnjaefdzuwp3h3mfluz6tm8h0dsq2ym3f77zsv0wrrszpmaqlezm3kt6ajdvlw".to_string(),
                ExampleWalletNetwork::Testnet(ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    _,
                )) => "ztestsapling1etnl5s47cqves0g5hk2dx5824rme4xv4aeauwzp4d6ys3qxykt5sw5rnaqh9syxry8vgxu60uhj".to_string(),
                ExampleWalletNetwork::Testnet(ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(
                    _,
                )) => "ztestsapling1h8l5mzlwhmqmd9x7ehquayqckzg6jwa6955f3w9mnkn5p5yfhqy04yz6yjrqfcztxx05xlh3prq".to_string(),
                ExampleWalletNetwork::Mainnet(ExampleMainnetWalletSeed::VTFCORFBCBPCTCFUPMEGMWBP(
                    _,
                )) => "zs1kgdrzfe6xuq3tg64vnezp3duyp43u7wcpgduqcpwz9wsnfqm4cecafu9qkmpsjtqxzf27n34z9k".to_string(),
                ExampleWalletNetwork::Mainnet(ExampleMainnetWalletSeed::HHCCLALTPCCKCSSLPCNETBLR(
                    _,
                )) => "zs1zgffhwsnh7efu4auv8ql9egteangyferp28rv8r7hmu76u0ee8mthcpflx575emx2zygqcuedzn".to_string(),
            },
            PoolType::Shielded(ShieldedProtocol::Orchard) => match self {
                ExampleWalletNetwork::Regtest(ExampleRegtestWalletSeed::HMVASMUVWMSSVICHCARBPOCT(
                    _,
                )) => "uregtest1wdukkmv5p5n824e8ytnc3m6m77v9vwwl7hcpj0wangf6z23f9x0fnaen625dxgn8cgp67vzw6swuar6uwp3nqywfvvkuqrhdjffxjfg644uthqazrtxhrgwac0a6ujzgwp8y9cwthjeayq8r0q6786yugzzyt9vevxn7peujlw8kp3vf6d8p4fvvpd8qd5p7xt2uagelmtf3vl6w3u8".to_string(),
                ExampleWalletNetwork::Regtest(ExampleRegtestWalletSeed::AAAAAAAAAAAAAAAAAAAAAAAA(
                    _,
                )) => "uregtest1zkuzfv5m3yhv2j4fmvq5rjurkxenxyq8r7h4daun2zkznrjaa8ra8asgdm8wwgwjvlwwrxx7347r8w0ee6dqyw4rufw4wg9djwcr6frzkezmdw6dud3wsm99eany5r8wgsctlxquu009nzd6hsme2tcsk0v3sgjvxa70er7h27z5epr67p5q767s2z5gt88paru56mxpm6pwz0cu35m".to_string(),
                ExampleWalletNetwork::Regtest(ExampleRegtestWalletSeed::AADAALACAADAALACAADAALAC(
                    _,
                )) => "uregtest1qtqr46fwkhmdn336uuyvvxyrv0l7trgc0z9clpryx6vtladnpyt4wvq99p59f4rcyuvpmmd0hm4k5vv6j8edj6n8ltk45sdkptlk7rtzlm4uup4laq8ka8vtxzqemj3yhk6hqhuypupzryhv66w65lah9ms03xa8nref7gux2zzhjnfanxnnrnwscmz6szv2ghrurhu3jsqdx25y2yh".to_string(),
                ExampleWalletNetwork::Testnet(ExampleTestnetWalletSeed::CBBHRWIILGBRABABSSHSMTPR(
                    _,
                )) => "utest17wwv8nuvdnpjsxtu6ndz6grys5x8wphcwtzmg75wkx607c7cue9qz5kfraqzc7k9dfscmylazj4nkwazjj26s9rhyjxm0dcqm837ykgh2suv0at9eegndh3kvtfjwp3hhhcgk55y9d2ys56zkw8aaamcrv9cy0alj0ndvd0wll4gxhrk9y4yy9q9yg8yssrencl63uznqnkv7mk3w05".to_string(),
                ExampleWalletNetwork::Testnet(ExampleTestnetWalletSeed::MSKMGDBHOTBPETCJWCSPGOPP(
                    _,
                )) => "utest19zd9laj93deq4lkay48xcfyh0tjec786x6yrng38fp6zusgm0c84h3el99fngh8eks4kxv020r2h2njku6pf69anpqmjq5c3suzcjtlyhvpse0aqje09la48xk6a2cnm822s2yhuzfr47pp4dla9rakdk90g0cee070z57d3trqk87wwj4swz6uf6ts6p5z6lep3xyvueuvt7392tww".to_string(),
                ExampleWalletNetwork::Mainnet(ExampleMainnetWalletSeed::VTFCORFBCBPCTCFUPMEGMWBP(
                    _,
                )) => "u1n5zgv8c9px4hfmq7cr9f9t0av6q9nj5dwca9w0z9jxegut65gxs2y4qnx7ppng6k2hyt0asyycqrywalzyasxu2302xt4spfqnkh25nevr3h9exc3clh9tfpr5hyhc9dwee50l0cxm7ajun5xs9ycqhlw8rd39jql8z5zlv9hw4q8azcgpv04dez5547geuvyh8pfzezpw52cg2qknm".to_string(),
                ExampleWalletNetwork::Mainnet(ExampleMainnetWalletSeed::HHCCLALTPCCKCSSLPCNETBLR(
                    _,
                )) => "u14lrpa0myuh5ax8dtyaj64jddk8m80nk2wgd3sjlu7g3ejwxs3qkfj5hntakjte8ena3qnk40ht0ats5ad0lcwhjtn9hak6733fdf33fhkl7umgqy2vtcfmhmca9pjdlrsz68euuw06swnl9uzzpadmvztd50xen4ruw738t995x7mhdcx3mjv7eh5hntgtvhtv6vgp9l885eqg6xpm8".to_string(),
            },
        }
    }
}
