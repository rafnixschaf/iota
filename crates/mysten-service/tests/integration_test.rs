// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use axum::{body::Body, http::Request};
use futures::TryStreamExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_mysten_service() {
    let app = mysten_service::get_mysten_service("itest", "0.0.0");

    let res = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    let body = res.into_body();
    let body_data = body
        .into_data_stream()
        .try_collect::<Vec<_>>()
        .await
        .unwrap()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    println!("{}", std::str::from_utf8(&body_data).unwrap());
    assert_eq!(
        &body_data[..],
        br#"{"name":"itest","version":"0.0.0","status":"up"}"#
    );
}
