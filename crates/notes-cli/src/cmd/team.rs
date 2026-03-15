use notes_core::StorageBackend;
use notes_storage_fs::FsStorage;

pub fn list(storage: &FsStorage, user_id: &str) -> anyhow::Result<()> {
    let teams = storage.list_teams_for_user(user_id)?;

    if teams.is_empty() {
        println!("(no teams)");
        return Ok(());
    }

    for team in &teams {
        let role = team
            .members
            .iter()
            .find(|m| m.user_id == user_id)
            .map(|m| format!("{:?}", m.role).to_lowercase())
            .unwrap_or_else(|| "?".into());
        println!("{:<20} {}  (your role: {role})", team.id, team.name);
    }

    Ok(())
}

pub fn members(storage: &FsStorage, team_id: &str) -> anyhow::Result<()> {
    let team = storage.load_team(team_id)?;
    println!("Team: {} ({})", team.name, team.id);
    println!();
    for member in &team.members {
        let role = format!("{:?}", member.role).to_lowercase();
        println!("  {:<30} {role}", member.user_id);
    }
    Ok(())
}
