use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use bevy::prelude::{App, Mut, PluginGroup, Real, Time};
use bevy::time::TimeUpdateStrategy;
use bevy::MinimalPlugins;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::netcode::generate_key;
use crate::prelude::client::{
    Authentication, Client, ClientConfig, InputConfig, InterpolationConfig, PredictionConfig,
    SyncConfig,
};
use crate::prelude::server::{NetcodeConfig, Server, ServerConfig};
use crate::prelude::*;

use crate::tests::protocol::{protocol, MyProtocol};

/// Helpers to setup a bevy app where I can just step the world easily

pub trait Step {
    /// Advance both apps by one frame duration
    fn frame_step(&mut self);

    /// Advance both apps by on fixed timestep duration
    fn tick_step(&mut self);
}

pub struct BevyStepper {
    pub client_app: App,
    pub server_app: App,
    pub frame_duration: Duration,
    /// fixed timestep duration
    pub tick_duration: Duration,
    pub current_time: std::time::Instant,
}

// Do not forget to use --features mock_time when using the LinkConditioner
impl BevyStepper {
    pub fn new(
        shared_config: SharedConfig,
        sync_config: SyncConfig,
        prediction_config: PredictionConfig,
        interpolation_config: InterpolationConfig,
        conditioner: LinkConditionerConfig,
        frame_duration: Duration,
    ) -> Self {
        // tracing_subscriber::FmtSubscriber::builder()
        //     .with_span_events(FmtSpan::ENTER)
        //     .with_max_level(tracing::Level::DEBUG)
        //     .init();

        // Use local channels instead of UDP for testing
        let addr = SocketAddr::from_str("127.0.0.1:0").unwrap();
        let io_1 = IoConfig::from_transport(TransportConfig::LocalChannel)
            .with_conditioner(conditioner.clone())
            .get_io();
        let (receiver_1, sender_1) = io_1.to_parts();

        let io_2 = IoConfig::from_transport(TransportConfig::LocalChannel)
            .with_conditioner(conditioner.clone())
            .get_io();
        let (receiver_2, sender_2) = io_2.to_parts();

        let io_1 = Io::new(addr, sender_2, receiver_1);
        let io_2 = Io::new(addr, sender_1, receiver_2);

        // Shared config
        let protocol_id = 0;
        let private_key = generate_key();
        let client_id = 111;

        // Setup server
        let mut server_app = App::new();
        server_app.add_plugins(MinimalPlugins.build());
        let netcode_config = NetcodeConfig::default()
            .with_protocol_id(protocol_id)
            .with_key(private_key);
        let config = ServerConfig {
            shared: shared_config.clone(),
            netcode: netcode_config,
            ping: PingConfig::default(),
        };
        let plugin_config = server::PluginConfig::new(config, io_1, protocol());
        let plugin = server::ServerPlugin::new(plugin_config);
        server_app.add_plugins(plugin);

        // Setup client
        let mut client_app = App::new();
        client_app.add_plugins(MinimalPlugins.build());
        let auth = Authentication::Manual {
            server_addr: addr,
            protocol_id,
            private_key,
            client_id,
        };
        let config = ClientConfig {
            shared: shared_config.clone(),
            input: InputConfig::default(),
            netcode: Default::default(),
            ping: PingConfig::default(),
            sync: sync_config,
            prediction: prediction_config,
            interpolation: interpolation_config,
        };
        let plugin_config = client::PluginConfig::new(config, io_2, protocol(), auth);
        let plugin = client::ClientPlugin::new(plugin_config);
        client_app.add_plugins(plugin);

        // Initialize Real time (needed only for the first TimeSystem run)
        let now = std::time::Instant::now();
        client_app
            .world
            .get_resource_mut::<Time<Real>>()
            .unwrap()
            .update_with_instant(now);
        server_app
            .world
            .get_resource_mut::<Time<Real>>()
            .unwrap()
            .update_with_instant(now);

        Self {
            client_app,
            server_app,
            frame_duration,
            tick_duration: shared_config.tick.tick_duration,
            current_time: now,
        }
    }

    pub fn client(&self) -> &Client<MyProtocol> {
        self.client_app.world.resource::<Client<MyProtocol>>()
    }

    pub fn client_mut(&mut self) -> Mut<Client<MyProtocol>> {
        self.client_app.world.resource_mut::<Client<MyProtocol>>()
    }

    fn server(&self) -> &Server<MyProtocol> {
        self.server_app.world.resource::<Server<MyProtocol>>()
    }
}

impl Step for BevyStepper {
    /// Advance the world by one frame duration
    fn frame_step(&mut self) {
        self.current_time += self.frame_duration;
        self.client_app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.current_time));
        self.server_app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.current_time));
        mock_instant::MockClock::advance(self.frame_duration);
        self.client_app.update();
        // sleep a bit to make sure that local io receives the packets
        // std::thread::sleep(Duration::from_millis(1));
        self.server_app.update();
        // std::thread::sleep(Duration::from_millis(1));
    }

    fn tick_step(&mut self) {
        self.current_time += self.tick_duration;
        self.client_app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.current_time));
        self.server_app
            .insert_resource(TimeUpdateStrategy::ManualInstant(self.current_time));
        mock_instant::MockClock::advance(self.tick_duration);
        self.client_app.update();
        self.server_app.update();
    }
}