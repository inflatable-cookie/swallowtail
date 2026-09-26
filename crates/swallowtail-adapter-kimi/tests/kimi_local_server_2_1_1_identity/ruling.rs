//! Exact ruling options for Research 354. Fails if an option is added,
//! dropped, or quietly rewritten into a claim.

use super::support::{IDENTITY, PROTOCOL, json, strings};

#[test]
fn the_decision_is_a_ruling_request_with_no_claim_change() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "ruling-request");
    assert_eq!(decision["claim_change"], false);
    assert_eq!(decision["latest_qualified_stays"], "0.39.1");
    assert_eq!(decision["posture_stays"], "qualified_only");
    assert_eq!(
        decision["claim_id_stays"],
        "kimi.local-server.executable-window-5"
    );
    assert_eq!(decision["rejected_gap"], "0.40.0..=2.1.1");
    assert_eq!(decision["widen_local_server_claim"], false);
    assert_eq!(decision["edit_local_server_selection_rs"], false);
    assert_eq!(decision["compatible_extension"], false);
    assert_eq!(decision["private_milestone"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["major_line_reset_is_same_product_same_axis"], true);
    assert_eq!(decision["major_line_reset_still_requires_ruling"], true);
    assert_eq!(strings(&decision["ruling_ids"]), ["A", "B", "C", "D"]);
}

#[test]
fn the_four_rulings_are_exact_and_none_is_taken() {
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
