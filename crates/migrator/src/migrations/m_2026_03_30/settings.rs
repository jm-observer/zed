use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn make_play_sound_when_agent_done_an_enum(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    if let Some(agent) = obj.get_mut("agent") {
        migrate_agent_with_profiles(agent)?;
    }

    Ok(())
}

fn migrate_agent_with_profiles(agent: &mut Value) -> Result<()> {
    migrate_play_sound(agent)?;

    if let Some(agent_object) = agent.as_object_mut() {
        if let Some(profiles) = agent_object.get_mut("profiles") {
            if let Some(profiles_object) = profiles.as_object_mut() {
                for (_profile_name, profile) in profiles_object.iter_mut() {
                    migrate_play_sound(profile)?;
                }
            }
        }
    }

    Ok(())
}

fn migrate_play_sound(agent: &mut Value) -> Result<()> {
    let Some(agent_object) = agent.as_object_mut() else {
        return Ok(());
    };

    let Some(play_sound) = agent_object.get_mut("play_sound_when_agent_done") else {
        return Ok(());
    };

    *play_sound = match play_sound {
        Value::Bool(true) => Value::String("always".to_string()),
        Value::Bool(false) => Value::String("never".to_string()),
        Value::String(s) if s == "never" || s == "when_hidden" || s == "always" => return Ok(()),
        _ => anyhow::bail!("Expected play_sound_when_agent_done to be a boolean"),
    };

    Ok(())
}
