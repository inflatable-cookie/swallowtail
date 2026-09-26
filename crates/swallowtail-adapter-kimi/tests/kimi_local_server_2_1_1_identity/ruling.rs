//! Exact ruling options for Research 354/357. Q-004 B is the taken ruling.

use super::support::{IDENTITY, PROTOCOL, json, strings};

#[test]
fn the_decision_takes_q004_b_and_widens_the_claim() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "q-004-b-ambient-host-extension");
    assert_eq!(decision["claim_change"], true);
    assert_eq!(decision["ruling_taken"], "B");
    assert_eq!(decision["question"], "Q-004");
    assert_eq!(decision["latest_qualified_becomes"], "2.1.1");
    assert_eq!(decision["posture_becomes"], "allow_unverified");
    assert_eq!(
        decision["claim_id_becomes"],
        "kimi.local-server.executable-window-6"
    );
    assert_eq!(
        decision["rejected_gap"],
        "0.39.2, 0.40.2, 0.41.1, 0.42.1, 0.43.2, 1.x, 2.0.3"
    );
    assert_eq!(decision["synthetic_later_unverified_newer"], "2.1.2");
    assert_eq!(decision["widen_local_server_claim"], true);
    assert_eq!(decision["edit_local_server_selection_rs"], true);
    assert_eq!(decision["compatible_extension"], true);
    assert_eq!(decision["private_milestone"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["major_line_reset_is_same_product_same_axis"], true);
    assert_eq!(decision["pin_disabled_tools"], false);
    assert_eq!(strings(&decision["ruling_ids"]), ["A", "B", "C", "D"]);
}

#[test]
fn q004_b_is_the_taken_ruling_and_the_other_three_stay_named() {
    let protocol = json(PROTOCOL);
    let options = protocol["ruling_options"]
        .as_array()
        .expect("ruling options are an array");
    assert_eq!(options.len(), 4);
    assert_eq!(options[0]["id"], "A");
    assert_eq!(options[0]["narrows_consumer_visible_guarantee"], true);
    assert_eq!(options[0]["contains_ambient_host"], false);
    assert_eq!(options[1]["id"], "B");
    assert_eq!(options[1]["withdraws_research_282_326_fail_closed"], true);
    assert_eq!(options[1]["taken"], true);
    assert_eq!(options[2]["id"], "C");
    assert_eq!(options[2]["mechanism_present_in_2_1_1_artifacts"], false);
    assert_eq!(options[2]["new_public_lifecycle"], true);
    assert_eq!(options[3]["id"], "D");
    assert_eq!(options[3]["exception_to_no_terminal_stop"], true);
    for option in options {
        let authorizes = option["authorizes"]
            .as_str()
            .expect("ruling authorizes text");
        assert!(!authorizes.is_empty());
    }
}
