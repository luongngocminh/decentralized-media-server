use std::collections::HashMap;

use media_server_gateway::ServiceKind;
use media_server_protocol::protobuf::cluster_gateway::ping_event::gateway_origin::Location;
use tokio::sync::{
    mpsc::{channel, Receiver, Sender},
    oneshot,
};

type QueryRequest = (ServiceKind, Option<(f32, f32)>, oneshot::Sender<Option<u32>>);

#[derive(Clone)]
pub struct GatewayDestSelector {
    tx: Sender<QueryRequest>,
}

impl GatewayDestSelector {
    pub async fn select(&self, kind: ServiceKind, location: Option<(f32, f32)>) -> Option<u32> {
        let (tx, rx) = oneshot::channel();
        self.tx.send((kind, location, tx)).await.ok()?;
        rx.await.ok()?
    }
}

pub struct GatewayDestRequester {
    rx: Receiver<QueryRequest>,
    req_seed: u64,
    reqs: HashMap<u64, oneshot::Sender<Option<u32>>>,
}

impl GatewayDestRequester {
    pub fn on_find_node_res(&mut self, req_id: u64, res: Option<u32>) {
        if let Some(tx) = self.reqs.remove(&req_id) {
            if tx.send(res).is_err() {
                log::error!("[GatewayDestRequester] answer for req_id {req_id} error");
            }
        }
    }

    pub fn recv(&mut self) -> Option<media_server_gateway::store_service::Control> {
        let (kind, location, tx) = self.rx.try_recv().ok()?;
        let req_id = self.req_seed;
        self.req_seed += 1;
        self.reqs.insert(req_id, tx);
        Some(media_server_gateway::store_service::Control::FindNodeReq(
            req_id,
            kind,
            location.map(|(lat, lon)| Location { lat, lon }),
        ))
    }
}

pub fn build_dest_selector() -> (GatewayDestSelector, GatewayDestRequester) {
    let (tx, rx) = channel(100);
    (
        GatewayDestSelector { tx },
        GatewayDestRequester {
            rx,
            req_seed: 0,
            reqs: HashMap::new(),
        },
    )
}

//TODO test
