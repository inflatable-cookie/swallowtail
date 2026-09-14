//! Frozen npm, platform, and executable identity for the Grok Build ACP run.

use super::support::{IDENTITY, assert_exact_string_set, json};
use std::collections::BTreeMap;

/// Previous qualified ACP ceiling; the run starts unverified above it.
pub(super) const PREVIOUS_CEILING: &str = "1.0.5";
/// Official npm stable the run targets.
pub(super) const OFFICIAL_STABLE: &str = "1.0.30";
/// Every compared hop, the ceiling plus every published stable after it.
pub(super) const COMPARED: &[&str] = &[
    "1.0.5", "1.0.6", "1.0.7", "1.0.8", "1.0.9", "1.0.10", "1.0.11", "1.0.12", "1.0.13", "1.0.14",
    "1.0.15", "1.0.16", "1.0.17", "1.0.18", "1.0.19", "1.0.20", "1.0.21", "1.0.22", "1.0.23",
    "1.0.24", "1.0.25", "1.0.26", "1.0.27", "1.0.28", "1.0.29", "1.0.30",
];

/// Every published stable after the ceiling, in order.
pub(super) const HOPS: &[&str] = &[
    "1.0.6", "1.0.7", "1.0.8", "1.0.9", "1.0.10", "1.0.11", "1.0.12", "1.0.13", "1.0.14", "1.0.15",
    "1.0.16", "1.0.17", "1.0.18", "1.0.19", "1.0.20", "1.0.21", "1.0.22", "1.0.23", "1.0.24",
    "1.0.25", "1.0.26", "1.0.27", "1.0.28", "1.0.29", "1.0.30",
];

pub(super) const EXECUTABLE_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.5",
        "3dfa7f04fbb5427a8fbead286591543aaecb478b3a0ab222c4329eca1a3b2f86",
    ),
    (
        "1.0.6",
        "14f76c7164a4bcad98d313afd300b7a3f2bd0a70880462477438c8af809dc678",
    ),
    (
        "1.0.7",
        "71dad3b49f169e9b97d65649bf7f56f10f072fac130c521c71110dabc60c6301",
    ),
    (
        "1.0.8",
        "b16e351ea3989fc2307b28032278c43cda1e043f0adadf8ab0f40d51128aba0d",
    ),
    (
        "1.0.9",
        "6010f2c38bc4d9b11f44cf4968294908fdb3390e0bcd59a05e76f8c4530d3afb",
    ),
    (
        "1.0.10",
        "c66b8b44b670d55bd20022e0ade5c8b449f76699d81f9861f3ad21992f604446",
    ),
    (
        "1.0.11",
        "7fd4681f61a65b1b19bc3774ebdea8df8b003bfae84a19f409b117b6ace4171e",
    ),
    (
        "1.0.12",
        "a7d2495b9721d6f4d01a2a1e74beb62cc5007276a76bbab0529a238b9373f165",
    ),
    (
        "1.0.13",
        "8669e0fdadceec25b8c159c355f427ffbd82583525d774b6ab1522197ea83b80",
    ),
    (
        "1.0.14",
        "90ef0f656cddd4ade316d3879af7b92927a07c4e2359d4f8f89af18b5e63d50d",
    ),
    (
        "1.0.15",
        "86ffac7a9547936f02e97dcd9a0ddb3f69486c7bac66ee9e1cbefc72d59b4635",
    ),
    (
        "1.0.16",
        "8d901060ef725068443fea122cc6f9e4818fb6724fc69ddeb39d464ed750e127",
    ),
    (
        "1.0.17",
        "be8b42718132e3c2239e779707aa37cb6794ac678cf667845a7711e52fa7f4d3",
    ),
    (
        "1.0.18",
        "99b77202286face91f27f97caca0590377b36300ccca99ff18e730228f7a5ca3",
    ),
    (
        "1.0.19",
        "d07bcf2a5babebb99301fb64d8aed00edc5980c98ae204bea9a91593c3ffcf68",
    ),
    (
        "1.0.20",
        "c68e058c3ac02d5af5060ec80d53dbfbb320b60d24ab69992f805af922b63039",
    ),
    (
        "1.0.21",
        "daac3e1cc56771b94605c39cc091b7d3c3d4ef0aea3a45658250200aa22e936b",
    ),
    (
        "1.0.22",
        "f17b8369d3283bb9ef3daa53bb95f68bfff91d605376eee1769dfbc798f7a9f6",
    ),
    (
        "1.0.23",
        "a697d8e96e93f2c687f15591208abaa4e127ed6d9bd0045712086279e8cdcca0",
    ),
    (
        "1.0.24",
        "4291021c1570a7c8610277a3d65490a5e54b50311e222c6b4614264f02a215b3",
    ),
    (
        "1.0.25",
        "9ef4a40ad60c6a5178a65caf39c2a148e6a98d0d2d350b10329dee34d9195d9c",
    ),
    (
        "1.0.26",
        "081f1d861e99bd738e2939ccc7d447d113f0e05fa253c6489a8918adaaf7d8b2",
    ),
    (
        "1.0.27",
        "6e85277b0432c894abcb325d9132bdc88c9715fa3089f65b7589963d5522ec80",
    ),
    (
        "1.0.28",
        "bfaa983f8dd4d300ce8277f52ec38257e42a6105d4df75492f6059a28e55cc5c",
    ),
    (
        "1.0.29",
        "2b6b44a2e7e72348c4fc65f3c427b1c47fe95cbcf9f43f204f826d382b7d85db",
    ),
    (
        "1.0.30",
        "d53b6e543e482716236748914331db50145c696ac7af91f1ebdedcf5654cfecb",
    ),
];

const GIT_HEADS: &[(&str, &str)] = &[
    ("1.0.5", "5115b46bc909ae5c7f5fc064455197440e796b6b"),
    ("1.0.6", "24c70bc7ffdd6f278fb0b75136c06282f9ae593c"),
    ("1.0.7", "325eae35b09b55a29ed340731edbdcab6adaa0f8"),
    ("1.0.8", "95f4d452703b4d0de2b799e3da2667aac509ee82"),
    ("1.0.9", "d5a34fd40c84c9e16ed18762aa2038cdbaf576d1"),
    ("1.0.10", "5992780042ca5ae88a709cfc01b089c6f9e3a536"),
    ("1.0.11", "6870d7b2fdb76aa42537250aaac7924c326546fa"),
    ("1.0.12", "ece2b556c27187f4cdbe9667a1d1151b47258f1b"),
    ("1.0.13", "5e9a58528b76b6128ee610059d79aecaa71b9b8d"),
    ("1.0.14", "dbb9bc1e773c56a541e74dfbed9220365b9fff90"),
    ("1.0.15", "9df779fea880ea7c33ced6c77b84a15b22a15560"),
    ("1.0.16", "a0239a2688c17ba173e62863d23ca44ffcd5b8f9"),
    ("1.0.17", "a549186d9d39311f2d3ee4208db62af8c65aa476"),
    ("1.0.18", "ea950872ad513f24be21388b287bf2dc765edba1"),
    ("1.0.19", "6b38df55f6b21c9fc453d1ee182845f47b2842da"),
    ("1.0.20", "df7ef6ad56ed56a879b0a52138cb8d6f7df04c4a"),
    ("1.0.21", "3aedf38d5cfb8f82bed8d8380ea4af0d4e584e8a"),
    ("1.0.22", "8f40483ca2a5db8e52ced38d5f1aa63c7bc735c0"),
    ("1.0.23", "7fa0ca2c9e6ab4b405f83b2a8dc5a0e1b724f06b"),
    ("1.0.24", "68e414c661e37965e3b8244c3d66e7fce2d0c644"),
    ("1.0.25", "f7e67d6988e2727a1c730e285661eca4aa5eb565"),
    ("1.0.26", "fadad3468632f90c38832f7f597dd3d0e6c793a4"),
    ("1.0.27", "a538938e5720026f3091318a7fd76c24ba01fcfb"),
    ("1.0.28", "cae16d2533b63d532ced411602293df06957d570"),
    ("1.0.29", "4c83f16c3e1071b22eccbb317db6f2801f811309"),
    ("1.0.30", "04b7ffed98c6943e3c736e6e8cfb7f979560638e"),
];

const PLATFORM_TARBALL_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.5",
        "94b9c4ec7574ef37daabc2fdc5824c5fd86cc84561f2726635fb233f66655192",
    ),
    (
        "1.0.6",
        "ed5905e549001b2eed47e4f49891c3a05e623f6c0e720de985eda777642f7436",
    ),
    (
        "1.0.7",
        "c4d683c08d5ca94046ff749df89a2daba037cbbda92928c9a47a4500e7e8d102",
    ),
    (
        "1.0.8",
        "23104e28a5f5cd57059be742282620bd735e2603f230d65c054e372e7c16d985",
    ),
    (
        "1.0.9",
        "4ebb4634c0e943f146bab2c0685337523b5480c881bab469aa275344b4e6a917",
    ),
    (
        "1.0.10",
        "730b8335a7e19b43d6f38c3247d38c349418ceddee4ff7ad16c94f8afa3c1ad3",
    ),
    (
        "1.0.11",
        "50e321db31395f7fe17927bdc981639a79b9e191782549675c8eb086cc88c839",
    ),
    (
        "1.0.12",
        "8b08df1fdb909afb35cd24b2c5f019343787d599b5dba17d99ef828c0e25c403",
    ),
    (
        "1.0.13",
        "97c4b3ecab67747dee270c2576a5fafde75962c56aec5c519c44d9b933d91363",
    ),
    (
        "1.0.14",
        "a0a6b941a912dbcd219948435b1b051438a131f7c2b896978834c42a50495439",
    ),
    (
        "1.0.15",
        "64b51fe138b7118794636d8a1edbd38871a06e12dbccdb70f1fb4b5c0bea6deb",
    ),
    (
        "1.0.16",
        "6cf18ff89e7b2f4caf453556d608a24a1f4816327921cdbf1890ea5f7d84913c",
    ),
    (
        "1.0.17",
        "be621867919c8f6165d326583e3ccd099cc9871040889689e54267ab4b37cc0b",
    ),
    (
        "1.0.18",
        "ecfe539f9ee714c0791936598a78e231275defad3a81304e7237342960c865f6",
    ),
    (
        "1.0.19",
        "8b49e881321a6f66da9ddcee7079a2c7d562fc7e8f3d51dcaed70fd7f03e72e6",
    ),
    (
        "1.0.20",
        "ea472302bc184079d8388e0c93bf9f5bd72eff66bb5ff54ff6b55915ca996746",
    ),
    (
        "1.0.21",
        "b502b36b6ab4911f2a7fd5152583de3ff4810e64a69b680a16f27b2e4a571d6a",
    ),
    (
        "1.0.22",
        "8bed2d18e41e42585af9df6b3fe38dadc9ec350e49d259536de1e9d98b7d2839",
    ),
    (
        "1.0.23",
        "7d823f5d758b26fc960fb9f0e5ce8ab668f8b4e0d0425d53ffdc2bbc81385127",
    ),
    (
        "1.0.24",
        "f0af5b2faaec27f79825cf8bfb3a79b0bf064d9a51014b993bfc22430b3ac942",
    ),
    (
        "1.0.25",
        "c1548be1210511909bc74e2f4264feb569166ffcfecf15123fda8ead250e0cc2",
    ),
    (
        "1.0.26",
        "f237c01a41f2993fa93a1cef1715ddbea16f571d2f8cc5293a17e2801bcc2964",
    ),
    (
        "1.0.27",
        "6c12734cadf73812a3a40dc10e9f8fda53d22f2f7311d224b87dde7119343e4e",
    ),
    (
        "1.0.28",
        "0782e98799dc03049aac7513fc70261dc2c82d0b237e1cc47ebcf76565650e4d",
    ),
    (
        "1.0.29",
        "da4a3756a367d5d342bd0885f6bbf73a497539ed2f810cc6ba6e914d345a35bf",
    ),
    (
        "1.0.30",
        "7e522683b99268fb44b5834909739bdea743fb8ed9fe9fa3186b6be56938b3ed",
    ),
];

const WRAPPER_TARBALL_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.5",
        "667bd0b99c0318e39d8dc4af501d1658df9ece3e7a7e2bd87b012c0bb25f6f31",
    ),
    (
        "1.0.6",
        "d816fd10c5ff1c2b0407c5c3ab39ecb07494b8fdb66a8f2dd7fe951a5b8e3538",
    ),
    (
        "1.0.7",
        "c76e89ffb6955b9a45d3d808cc22daff3d41a62bb1b66187581afca9e71afae8",
    ),
    (
        "1.0.8",
        "858ca012c405b7c0b63ca3ffc0ea59cff7e3e46ab32873eb7e602c404f7a25f3",
    ),
    (
        "1.0.9",
        "46f755e672d16bcfd3f0dd5a9cf62897e04dec3cf30628154a785f7e4ba9901f",
    ),
    (
        "1.0.10",
        "99808a96748fd28fd971074126e516965adacce61221efcb94f281aa75c17ffe",
    ),
    (
        "1.0.11",
        "83051ac17236d6c97a052765decfc6d006f6248188bb6de5227e713d21592fca",
    ),
    (
        "1.0.12",
        "e8b8b45268816a6ab59c9a0b306b3d3025bccaafb5db18e7c80edf4ee764cb8d",
    ),
    (
        "1.0.13",
        "97aee4a07bd358233616a97a63d7732539721e0f9ea7ffad4f018f15b0cad0ee",
    ),
    (
        "1.0.14",
        "eac00528816d7fe6d9b38fa9071b234369519a2c93eb5c9f572b0dfdb83182bb",
    ),
    (
        "1.0.15",
        "059c491a28c5c067c4be48aa1bcc9566155bdeac1eb3901795a4ebdcad8412a3",
    ),
    (
        "1.0.16",
        "d8d80a41a7cfa518ec32fcee793b8aa24d37d4c031bdd393db7e0346bc902c5c",
    ),
    (
        "1.0.17",
        "6c45171fa1df018aeb3b8447ca2cea09717bec86e01e0749f6a3f9d3f95be248",
    ),
    (
        "1.0.18",
        "90ae26f55e32abe1ddb2e27f4f31f7e7b40ed2da46dec0c15dc90874e4df7449",
    ),
    (
        "1.0.19",
        "cb324aa4f214325c0d858fa689da45c095c92d2d52db37f5bcb2a0c55dd1be00",
    ),
    (
        "1.0.20",
        "74a2b2dbbc1971a1ab1a995413803c2d8c9750c315ff25792c5535e20e41c972",
    ),
    (
        "1.0.21",
        "8b1ef8361c77d03ac9fcec0b8854f3249a86c9ac3404511fca3da7ac5dba83a4",
    ),
    (
        "1.0.22",
        "583f1d45004bc1763a16490233bab84c9cc53700dd4eb57767421f7c5ca37cd7",
    ),
    (
        "1.0.23",
        "859ff58ef001a77249adcc35da84e0aeb14c2baf482e4482d123e6e39581e027",
    ),
    (
        "1.0.24",
        "31984364f75c3cf58b0b14a86e4dedea59fa0d4f5fc0b28e35a439123b2647fd",
    ),
    (
        "1.0.25",
        "74402844a945d6be6e065e3e0fa2881957aba9af36581f8cc2fb698f73be8ab2",
    ),
    (
        "1.0.26",
        "ffe0809819ca01f8562f33f9f76fa31ed5ff477dd414d211f0531a4ab683fbaa",
    ),
    (
        "1.0.27",
        "1ea56aaa23b1486be8abd4d10c9ad2b0b12351f29df05564faf29568ff2b4d83",
    ),
    (
        "1.0.28",
        "3cd21624ee1a75b1791f53591bedd006a737c847df38ad3dad98585c62ca4cc8",
    ),
    (
        "1.0.29",
        "9adf32651ba3be6c50619ad0f47ab3d72ba9b5196f0e7efd7732e6b64cc57fbc",
    ),
    (
        "1.0.30",
        "c57e7106e1f18e9d41677d06836a0abb3a498ae353763dacab3a1da473351628",
    ),
];

const MODEL_DOCUMENT_DIGESTS: &[(&str, &str)] = &[
    (
        "1.0.5",
        "264d0f644b0c435b0be869c081ac08231a205ccacecc6a46df2a7426f550748a",
    ),
    (
        "1.0.6",
        "264d0f644b0c435b0be869c081ac08231a205ccacecc6a46df2a7426f550748a",
    ),
    (
        "1.0.7",
        "264d0f644b0c435b0be869c081ac08231a205ccacecc6a46df2a7426f550748a",
    ),
    (
        "1.0.8",
        "264d0f644b0c435b0be869c081ac08231a205ccacecc6a46df2a7426f550748a",
    ),
    (
        "1.0.9",
        "264d0f644b0c435b0be869c081ac08231a205ccacecc6a46df2a7426f550748a",
    ),
    (
        "1.0.10",
        "264d0f644b0c435b0be869c081ac08231a205ccacecc6a46df2a7426f550748a",
    ),
    (
        "1.0.11",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.12",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.13",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.14",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.15",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.16",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.17",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.18",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.19",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.20",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.21",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.22",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.23",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.24",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.25",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.26",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.27",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.28",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.29",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
    (
        "1.0.30",
        "9d6924ec760a94f91902f60adb8bcf2cd9d3ab86891a1c83e8c01a092e56490a",
    ),
];

fn table<'a>(pairs: &'a [(&'a str, &'a str)]) -> BTreeMap<&'a str, &'a str> {
    pairs.iter().copied().collect()
}

#[test]
fn official_channel_freezes_every_published_stable_hop() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["npm_package"], "@xai-official/grok");
    assert_eq!(identity["npm_dist_tags"]["latest"], OFFICIAL_STABLE);
    assert_eq!(identity["npm_dist_tags"]["alpha"], "1.0.31");
    assert_eq!(identity["previous_ceiling"], PREVIOUS_CEILING);
    assert_eq!(identity["official_stable"], OFFICIAL_STABLE);
    assert_eq!(identity["first_unpublished_stable_after_latest"], "1.0.32");
    assert_exact_string_set(&identity["published_stables_after_ceiling"], HOPS);
    assert_exact_string_set(&identity["alpha_after_ceiling"], &["1.0.31"]);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified_before"],
        PREVIOUS_CEILING
    );
    assert_eq!(
        identity["claim_at_observation"]["posture"],
        "allow_unverified"
    );
}

#[test]
fn host_binary_is_the_official_latest_platform_artifact() {
    let identity = json(IDENTITY);
    let host = &identity["host"];
    assert_eq!(host["version"], OFFICIAL_STABLE);
    assert_eq!(host["source_revision"], "04b7ffed98c6");
    assert_eq!(host["cli"], "grok 1.0.30 (04b7ffed98c6) [stable]");
    assert_eq!(host["installed"], true);
    assert_eq!(host["equals_official_darwin_arm64_1_0_30_executable"], true);
    assert_eq!(
        host["binary_sha256"].as_str().unwrap(),
        table(EXECUTABLE_DIGESTS)[OFFICIAL_STABLE]
    );
}

#[test]
fn every_hop_pins_exact_wrapper_platform_and_executable_identity() {
    let identity = json(IDENTITY);
    let hops = identity["hops"].as_array().expect("hops array");
    assert_eq!(hops.len(), COMPARED.len());
    let executables = table(EXECUTABLE_DIGESTS);
    let git_heads = table(GIT_HEADS);
    let platform_tars = table(PLATFORM_TARBALL_DIGESTS);
    let wrapper_tars = table(WRAPPER_TARBALL_DIGESTS);
    let model_docs = table(MODEL_DOCUMENT_DIGESTS);
    let mut seen_executables = BTreeMap::new();
    for (index, entry) in hops.iter().enumerate() {
        let version = entry["version"].as_str().expect("version");
        assert_eq!(version, COMPARED[index]);
        assert_eq!(entry["git_head"], git_heads[version]);
        assert_eq!(entry["source_revision"], &git_heads[version][..12]);
        assert_eq!(entry["wrapper"]["tarball_sha256"], wrapper_tars[version]);
        assert_eq!(entry["platform"]["tarball_sha256"], platform_tars[version]);
        assert_eq!(entry["platform"]["executable_sha256"], executables[version]);
        assert_eq!(entry["model_document_sha256"][0], model_docs[version]);
        let sha = entry["platform"]["executable_sha256"].as_str().unwrap();
        assert_eq!(sha.len(), 64);
        assert!(sha.bytes().all(|byte| byte.is_ascii_hexdigit()));
        let size = entry["platform"]["executable_size"].as_u64().unwrap();
        assert!(size > 0);
        let integrity = entry["wrapper"]["integrity"].as_str().unwrap();
        assert!(integrity.starts_with("sha512-"), "{integrity}");
        assert_eq!(entry["wrapper"]["shasum"].as_str().unwrap().len(), 40);
        let platforms = entry["platform"]["other_platform_integrities"]
            .as_object()
            .expect("platform integrities");
        assert_eq!(platforms.len(), 6);
        for (name, value) in platforms {
            assert!(value.as_str().unwrap().starts_with("sha512-"), "{name}");
        }
        assert!(
            seen_executables.insert(sha.to_owned(), version).is_none(),
            "each hop ships a distinct executable"
        );
    }
}

#[test]
fn identity_decision_names_a_compatible_extension_before_the_claim_card() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "grok-build.acp-v1.cached-token-model-4-6-v3"
    );
    assert_eq!(decision["raise_latest_qualified_to"], OFFICIAL_STABLE);
    assert_eq!(decision["keep_baseline"], "0.2.114");
    assert_eq!(decision["keep_0_2_window"], true);
    assert_eq!(decision["keep_1_0_4_floor"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["model_binding"], "grok-4.6");
    assert_eq!(decision["mid_gap_0_2_118_through_0_2_121"], "incompatible");
    assert_eq!(decision["unprobed_1_0_0_through_1_0_3"], "incompatible");
    assert_eq!(
        decision["catalogue_claim_unchanged"],
        "exact 1.0.25 under QualifiedOnly"
    );
    assert_eq!(
        decision["registered_tool_claim_unchanged"],
        "exact 1.0.4 and 1.0.5 only"
    );
    assert_eq!(decision["later_unverified_after_qualification"], "1.0.31");
    for flag in [
        "provider_prompt_sent",
        "live_acp_session",
        "downloaded_artifacts_executed",
        "host_install_changed",
    ] {
        assert_eq!(decision[flag], false, "{flag} must stay false");
    }
    assert_exact_string_set(&decision["qualify_intermediates"], HOPS);
}
