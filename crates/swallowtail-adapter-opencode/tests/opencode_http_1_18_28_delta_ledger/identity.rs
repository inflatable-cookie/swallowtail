use super::{IDENTITY, assert_exact_strings, json};

const EXPECTED_HOPS: [[&str; 8]; 9] = [
    [
        "1.18.20",
        "2026-08-21T08:09:54.390Z",
        "sha512-8c2yJ/Oe1qFi9KYE0KS9WCyy6O1QtI9odzBmBWGOeyOgXTn/hGOwCp/fgcHY2qVQ2TVgkQXze7jXjJ6AFyeU0Q==",
        "79ace165fba034da1599fb3411691611228c409a",
        "d7af626824cab417d9c5c12e5c0187e506f1c903ea93bd8e4b1615be16305d2a",
        "7248bc1964b13fa67e601733f89ee9dc6dfa0563",
        "2026-08-21T08:09:31Z",
        "10129b7a233d8ea227fe8a65c158d3df4adc3d1296e3af5a136d94080b25a630",
    ],
    [
        "1.18.21",
        "2026-08-21T14:51:28.536Z",
        "sha512-BxQyxpD0y2X0sXJUKLOooXVmi9QIoeKPtdH68r7QRiqXJ/YulK1MQvSe8KyA8183zoPV0G6JAtgz1OqmE3OGUw==",
        "26cf454879fd325fc43bce0ce82ee75a24cdd112",
        "62eae64938cfc9ae1ee74222c6967ab081a2b5037bce947b0a3ba0b35694cd38",
        "826d9ad46a22bef0294998e08daa3c4904fea28f",
        "2026-08-21T14:51:11Z",
        "387ad0d4ef4364c00100fafaf90f80cff6858d7055bddd920aeacbab9b34a49d",
    ],
    [
        "1.18.22",
        "2026-08-24T14:37:31.515Z",
        "sha512-cSIGgB6tX3P+8k4X2ZzuJ9ojClfo01ou0ck2ocRDNXLfhVJy4XVLk/WCsK/m+Venbz3p2qCNxpAFNb47Gj4tLQ==",
        "13386b2af20e062d07b1500853e43e4681d2ddf8",
        "920ce17f8d9f24865d161e26d7e3e5121aa386b5a221374109cc118d52cce4e7",
        "47b6b6f5f4f9b42d2bce7af1c4e5bf6efaf22ba7",
        "2026-08-24T14:37:19Z",
        "b777d4f92268168b9386b79eca0faa72a92367773fb6d81197cccf886901a3b9",
    ],
    [
        "1.18.23",
        "2026-08-25T06:33:58.860Z",
        "sha512-3NkT0XINL7d0HYkTyGV1SPChHXhvRgKqNaTgKRTGb0TXUWszXA7MW/y3zMZw29y1AQuUDAzRvVYmQ9KGRQhroA==",
        "312261b7162ce58f69d4a0073a63856f7b967f5a",
        "4ba5929a9bf726bde96c9566b6902a58dcb2b338503a7b380c2fbf503cb1f0fe",
        "ef2880f379129aa048be9e9353e30aa168d42c17",
        "2026-08-25T06:30:49Z",
        "7b621e56e9e9162464f2524d88810f8e0e0036cf29cdadb35c574384eba3e6cf",
    ],
    [
        "1.18.24",
        "2026-08-28T04:09:47.526Z",
        "sha512-PIV4ud2cqFQw6Lm+onvxQrQo+wCFYnA+6TDryeDJ6PF38t2ujzeeKPzlxZ1vCmZ6p7AixtfgJu4iUovDVt4GZQ==",
        "74f0fedc34c5c78d856399249b3530944f297187",
        "b2adda2c2ba0bb7cf5e6a12cc090c56c969613acc9a464e75691cc5b316fa166",
        "57e80556975fca613a116447ddecc8dcbc1f33b2",
        "2026-08-28T04:10:14Z",
        "5acb57c53b39221ea8b8c8f55acd82c0a402a556a5da164c94b4d879756bb0ac",
    ],
    [
        "1.18.25",
        "2026-08-28T05:57:34.071Z",
        "sha512-pS4RKJ9eKwU7Dp5G5pdj1rhMnpG5APixXzfTKNoFqv9aFVI36Rnza2jESvKifxyPZlsA65MQB03WCArY0EK6mg==",
        "0649bb7833e3db91ab1435e9c8a8da75f8eae162",
        "9be29b0858b3c9bb1214569f1d8e48a783956c8f5093cc6dcd86717e2cd8c5a3",
        "cb7d8b2f5e44876ef98b661dc10590c915af3a9f",
        "2026-08-28T05:58:20Z",
        "44e9530d7be172005c7d60aef317440eecb85d557d94cce7fa35c5a7b9d9da0b",
    ],
    [
        "1.18.26",
        "2026-09-01T21:51:32.962Z",
        "sha512-XFPIj/yJZN8eBi4+uTjsnYAd/QezCM+/OUa3JbtL7tKQF4fGHR4onZx2d6oUGqbVs4CNONQ8QFKejeW+qVJIEA==",
        "ffdac376880c2a6b8a58fcbbc1445dbb1f79aa09",
        "d3eabbc23b5ef7e9383697c689b3b919f504d2cba36dcabe1ccc8de67380acb5",
        "774cc7c1914e4329eefde5a669f938b0cf566661",
        "2026-09-01T21:52:15Z",
        "a2ff47601072064f04263a97cce014c5b8d0692f7beaa7c7427ac02362d6c3d0",
    ],
    [
        "1.18.27",
        "2026-09-02T21:39:46.111Z",
        "sha512-5xrG2gQEwV2sLus30SZX9GyLbPX3z57BCxddedDM0wx1bgnwlHVLOS/FD2uve7fEZlmkr7KYFbvs65ySz1rwzA==",
        "4c3c54227e0d3496b0914d19c98e70fd9c088108",
        "d1746d5dab3997f971fa643c0e1e5e553e9e18be9530f27a2a562bc19f610300",
        "4b7e19e315cca414121ba1d61523fef74bb3ae8b",
        "2026-09-02T21:41:01Z",
        "3d3851762d41da2dafe3be39d3b17a222426747e9b49e5e87d2a88b46b0866f1",
    ],
    [
        "1.18.28",
        "2026-09-04T15:40:40.661Z",
        "sha512-T7FvoXv0gT0fKuEdiomQuke2KbIFb8B8xj9L/4ZFnMIt70GnU1uUj0y/OkhDZ2dCgterQLdRXrdhrLLZ4lyv6Q==",
        "32b51b0e9e5054d2ea62b7c9983904234d3dcf5e",
        "ae46e3653cb85edb4eab36127f289ba71833d70c0efb56992f99eca2940117c4",
        "22006d97652839999596a34a48ff6be7dbb40c6e",
        "2026-09-04T15:38:23Z",
        "8eea501a6a00cbebe524af7c3248c0bfc56290f444671903e32aa6b799ee6616",
    ],
];

#[test]
fn official_hop_identity_is_exact_and_identity_first() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "opencode.server");
    assert_eq!(identity["version"], "1.18.28");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["github_latest_tag"], "v1.18.28");
    assert_eq!(identity["official_channels_agree"], true);
    assert_eq!(identity["host"]["version"], "1.18.18");
    assert_eq!(
        identity["host"]["sha256"],
        "4f5979c2dadb06fbff1335335afaaea274e58f92e79aa43cf2ed98618d555422"
    );
    assert_eq!(identity["host"]["size"], 143_182_562);
    assert_eq!(identity["host"]["signature"], "adhoc-linker-signed");
    assert_eq!(identity["unpublished_next"], "1.18.29");
    assert_exact_strings(
        &identity["published_stables_from_previous_ceiling"],
        &[
            "1.18.21", "1.18.22", "1.18.23", "1.18.24", "1.18.25", "1.18.26", "1.18.27", "1.18.28",
        ],
    );

    let hops = identity["official_hops"].as_array().expect("hop array");
    assert_eq!(hops.len(), EXPECTED_HOPS.len());
    for (hop, expected) in hops.iter().zip(EXPECTED_HOPS.iter()) {
        for (key, value) in [
            ("version", expected[0]),
            ("npm_published_at", expected[1]),
            ("npm_integrity", expected[2]),
            ("npm_shasum", expected[3]),
            ("npm_tarball_sha256", expected[4]),
            ("github_tag_commit", expected[5]),
            ("github_release_published_at", expected[6]),
            ("source_archive_sha256", expected[7]),
        ] {
            assert_eq!(hop[key], value, "{key} drifted for {}", expected[0]);
        }
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.28");
    assert_eq!(decision["claim_changed_in_identity_card"], false);
    assert_eq!(decision["downloaded_artifact_executed"], false);
    assert_eq!(decision["host_install_changed"], false);
    assert_eq!(decision["claim_card"], "g05 batch card 078");
}
