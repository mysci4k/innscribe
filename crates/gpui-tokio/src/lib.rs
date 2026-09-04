// Copyright (c) 2022-2026 Zed Industries, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Derived from Zed Industries (gpui_tokio):
// https://github.com/zed-industries/zed/blob/main/crates/gpui_tokio/src/gpui_tokio.rs
// Modified for use with gpui-pre / gpui-kit.

use gpui_kit::{App, AppContext, Global, ReadGlobal, Task};
use std::future::Future;

pub use tokio::task::JoinError;

pub fn init(cx: &mut App) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("Failed to initialize Tokio");

    let handle = runtime.handle().clone();
    cx.set_global(GlobalTokio {
        owned_runtime: Some(runtime),
        handle,
    });
}

pub fn init_from_handle(cx: &mut App, handle: tokio::runtime::Handle) {
    cx.set_global(GlobalTokio {
        owned_runtime: None,
        handle,
    });
}

struct GlobalTokio {
    owned_runtime: Option<tokio::runtime::Runtime>,
    handle: tokio::runtime::Handle,
}

impl Global for GlobalTokio {}

impl Drop for GlobalTokio {
    fn drop(&mut self) {
        if let Some(runtime) = self.owned_runtime.take() {
            runtime.shutdown_background();
        }
    }
}

pub struct AbortOnDrop(tokio::task::AbortHandle);

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}

pub struct Tokio {}

impl Tokio {
    pub fn spawn<C, Fut, R>(cx: &C, f: Fut) -> Task<Result<R, JoinError>>
    where
        C: AppContext,
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        cx.read_global(|tokio: &GlobalTokio, cx| {
            let join_handle = tokio.handle.spawn(f);
            let abort_handle = join_handle.abort_handle();
            let abort_guard = AbortOnDrop(abort_handle);

            cx.background_spawn(async move {
                let _guard = abort_guard;

                join_handle.await
            })
        })
    }

    pub fn spawn_result<C, Fut, R>(cx: &C, f: Fut) -> Task<anyhow::Result<R>>
    where
        C: AppContext,
        Fut: Future<Output = anyhow::Result<R>> + Send + 'static,
        R: Send + 'static,
    {
        cx.read_global(|tokio: &GlobalTokio, cx| {
            let join_handle = tokio.handle.spawn(f);
            let abort_handle = join_handle.abort_handle();

            cx.background_spawn(async move {
                let _guard = AbortOnDrop(abort_handle);

                join_handle.await?
            })
        })
    }

    pub fn handle(cx: &App) -> tokio::runtime::Handle {
        GlobalTokio::global(cx).handle.clone()
    }
}
