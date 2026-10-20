use levels_core::config::MAX_COLUMN_SURFACES;
use levels_ffi::api::draw::{lv_draw_list, lv_report_timing, lv_submit_column};
use levels_ffi::types::{LV_MAX_COLUMN_SURFACES, LvCamera, LvDrawView, LvStatus};

use crate::support::LvSession;

#[test]
fn max_surfaces_matches_core() {
    assert_eq!(LV_MAX_COLUMN_SURFACES as usize, MAX_COLUMN_SURFACES);
}

#[test]
fn null_arguments_are_invalid() {
    let session = LvSession::new();
    let sample = LvSession::under_feet(0, 0);
    let mut out = LvDrawView::default();

    assert_eq!(lv_draw_list(std::ptr::null(), 0, std::ptr::null(), &mut out), LvStatus::InvalidArgument);
    assert_eq!(lv_draw_list(session.handle, 0, std::ptr::null(), std::ptr::null_mut()), LvStatus::InvalidArgument);
    assert_eq!(lv_submit_column(std::ptr::null(), 0, 0, &sample, 1), LvStatus::InvalidArgument);
    assert_eq!(lv_submit_column(session.handle, 0, 0, std::ptr::null(), 1), LvStatus::InvalidArgument);
    assert_eq!(lv_report_timing(std::ptr::null(), 1.0, 1.0, 0), LvStatus::InvalidArgument);
}

#[test]
fn null_samples_with_zero_count_is_an_empty_column() {
    let session = LvSession::new();

    assert_eq!(lv_submit_column(session.handle, 0, 0, std::ptr::null(), 0), LvStatus::Ok);
}

#[test]
fn oversized_column_is_rejected() {
    let session = LvSession::new();
    let samples = vec![LvSession::under_feet(0, 0); LV_MAX_COLUMN_SURFACES as usize + 1];

    assert_eq!(session.submit(10, -20, &samples), LvStatus::InvalidArgument);
}

#[test]
fn submit_then_draw_returns_label() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));

    assert_eq!(session.submit(10, -20, &[LvSession::under_feet(9, 0)]), LvStatus::Ok);
    let (code, view) = session.draw();

    assert_eq!(code, LvStatus::Ok);
    assert_eq!(view.count, 1);
    // SAFETY: the view is valid for count elements until the next lv_draw_list on this handle
    let label = unsafe { *view.labels };
    assert_eq!(label.value, 9);
    assert!((label.x - 10.5).abs() < 1e-5 && (label.z - -19.5).abs() < 1e-5);
}

#[test]
fn draw_view_carries_rotation_and_scale() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("config.json");
    std::fs::write(&file, r#"{"label":{"scale":0.5}}"#).unwrap();
    let session = LvSession::with_config(&file);
    session.turn_on();
    session.frame(&levels_ffi::types::LvFrameStatus { yaw: 90.0, ..LvSession::in_world(1.0 / 60.0) });

    let (_, view) = session.draw();

    assert_eq!(view.rotation, levels_core::services::LabelRotation::for_yaw(90.0));
    assert_eq!(view.scale, 0.5);
}

#[test]
fn views_stay_valid_until_next_call() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(9, 0)]);
    let (_, held) = session.draw();
    // SAFETY: the view is valid until the next lv_draw_list on this handle
    let before = unsafe { *held.labels };

    session.frame(&LvSession::in_world(2.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(3, 0)]);

    // SAFETY: no lv_draw_list call was made since the view was returned
    let after = unsafe { *held.labels };
    assert_eq!(before, after);
}

#[test]
fn game_tags_past_every_label_leave_none() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(9, 0)]);

    let (code, view) = session.draw_beside(100_000);

    assert_eq!(code, LvStatus::Ok);
    assert_eq!(view.count, 0);
}

#[test]
fn camera_facing_away_hides_the_label() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(9, 0)]);
    let away = LvCamera { x: 20.5, y: 66.0, z: -19.5, forward_x: 1.0, forward_y: 0.0, forward_z: 0.0 };
    let toward = LvCamera { forward_x: -1.0, ..away };

    let (_, hidden) = session.draw_from(0, &away);
    let (_, shown) = session.draw_from(0, &toward);

    assert_eq!((hidden.count, shown.count), (0, 1));
}

#[test]
fn camera_without_a_direction_hides_nothing() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(9, 0)]);
    let still = LvCamera { x: 20.5, y: 66.0, z: -19.5, forward_x: 0.0, forward_y: 0.0, forward_z: 0.0 };

    assert_eq!(session.draw_from(0, &still).1.count, 1);
}

#[test]
fn hidden_hud_draws_nothing() {
    let session = LvSession::new();
    session.turn_on();
    session.frame(&LvSession::in_world(1.0 / 60.0));
    session.submit(10, -20, &[LvSession::under_feet(9, 0)]);

    session.frame(&levels_ffi::types::LvFrameStatus { hud_hidden: 1, ..LvSession::in_world(2.0 / 60.0) });

    assert_eq!(session.draw_beside(0).1.count, 0);
}
