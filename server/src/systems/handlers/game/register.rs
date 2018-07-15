use super::*;
use dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	let builder = builder
		.with::<on_spectate_event::SetSpectateFlag>()
		.with::<on_spectate_event::SendKillPacket>()
		.with::<on_spectate_event::SendSpectatePacket>()
		.with::<on_spectate_event::SendTimerEvent>()
		.with::<on_spectate_event::SetSpectateTarget>()
		.with::<on_player_killed::SetRespawnTimer>()
		.with::<on_player_killed::DisplayMessage>()
		.with::<on_player_killed::UpdateScore>()
		.with::<on_join::InitConnection>()
		.with::<on_join::InitKillCounters>()
		.with::<on_join::InitJoinTime>()
		.with::<on_join::InitEarnings>()
		.with::<on_join::InitTraits>()
		.with::<on_join::InitState>()
		.with::<on_join::InitName>()
		.with::<on_join::InitLimiters>()
		.with::<on_join::InitTransform>()
		.with::<on_join::InitStealthTime>()
		.with::<on_join::InitLastRepelTime>()
		.with::<on_join::SendPlayerNew>()
		.with::<on_join::SendLogin>()
		.with::<on_join::SendPlayerLevel>()
		.with::<on_join::SendScoreUpdate>()
		.with::<on_join::UpdatePlayersGame>()
		.with::<on_missile_fire::SendPlayerFire>()
		.with::<on_missile_fire::SetLastShot>()
		.with::<on_player_hit::InflictDamage>()
		.with::<on_player_hit::SendPacket>()
		// Needs to be after InflictDamage
		.with::<PlayerKilledCleanup>();

	let builder = on_chat_throttled::register(builder);

	timer::register(builder)
}
