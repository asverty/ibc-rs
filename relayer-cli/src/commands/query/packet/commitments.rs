use abscissa_core::clap::Parser;
use abscissa_core::{Command, Runnable};
use ibc::core::ics04_channel::packet::Sequence;
use serde::Serialize;

use ibc::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use ibc::Height;
use ibc_relayer::chain::counterparty::commitments_on_chain;

use crate::cli_utils::spawn_chain_runtime;
use crate::conclude::Output;
use crate::error::Error;
use crate::prelude::*;

#[derive(Serialize, Debug)]
struct PacketSeqs {
    height: Height,
    seqs: Vec<Sequence>,
}

#[derive(Clone, Command, Debug, Parser, PartialEq)]
pub struct QueryPacketCommitmentsCmd {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help = "Identifier of the port to query"
    )]
    port_id: PortId,

    #[clap(
        long = "channel",
        alias = "chan",
        required = true,
        value_name = "CHANNEL_ID",
        help = "Identifier of the channel to query"
    )]
    channel_id: ChannelId,
}

impl QueryPacketCommitmentsCmd {
    fn execute(&self) -> Result<PacketSeqs, Error> {
        let config = app_config();

        debug!("Options: {:?}", self);

        let chain = spawn_chain_runtime(&config, &self.chain_id)?;

        commitments_on_chain(&chain, &self.port_id, &self.channel_id)
            .map_err(Error::supervisor)
            .map(|(seqs_vec, height)| PacketSeqs {
                height,
                seqs: seqs_vec,
            })
    }
}

// cargo run --bin hermes -- query packet commitments --chain ibc-0 --port transfer --channel ibconexfer --height 3
impl Runnable for QueryPacketCommitmentsCmd {
    fn run(&self) {
        match self.execute() {
            Ok(p) => Output::success(p).exit(),
            Err(e) => Output::error(format!("{}", e)).exit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryPacketCommitmentsCmd;

    use std::str::FromStr;

    use abscissa_core::clap::Parser;
    use ibc::core::ics24_host::identifier::{ChainId, PortId, ChannelId};

    #[test]
    fn test_query_packet_commitments() {
        assert_eq!(
            QueryPacketCommitmentsCmd{ chain_id: ChainId::from_string("chain_id"), port_id: PortId::from_str("port_id").unwrap(), channel_id: ChannelId::from_str("channel-07").unwrap() },
            QueryPacketCommitmentsCmd::parse_from(&["test", "--chain", "chain_id", "--port", "port_id", "--chan", "channel-07"])
        )
    }

    #[test]
    fn test_query_packet_commitments_no_chan() {
        assert!(QueryPacketCommitmentsCmd::try_parse_from(&["test", "--chain", "chain_id", "--port", "port_id"]).is_err())
    }

    #[test]
    fn test_query_packet_commitments_no_port() {
        assert!(QueryPacketCommitmentsCmd::try_parse_from(&["test", "--chain", "chain_id", "--chan", "channel-07"]).is_err())
    }

    #[test]
    fn test_query_packet_commitments_no_chain() {
        assert!(QueryPacketCommitmentsCmd::try_parse_from(&["test", "--port", "port_id", "--chan", "channel-07"]).is_err())
    }
}
