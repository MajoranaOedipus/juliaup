use crate::config_file::JuliaupConfig;
use crate::config_file::{load_mut_config_db, save_config_db, JuliaupConfigChannel};
use crate::global_paths::GlobalPaths;
use crate::jsonstructs_versionsdb::JuliaupVersionDB;
#[cfg(not(windows))]
use crate::operations::create_symlink;
use crate::operations::garbage_collect_versions;
use crate::operations::{install_version, update_version_db};
use crate::versions_file::load_versions_db;
use anyhow::{anyhow, bail, Context, Result};

fn update_channel(
    config_db: &mut JuliaupConfig,
    channel: &String,
    version_db: &JuliaupVersionDB,
    ignore_non_updatable_channel: bool,
    paths: &GlobalPaths,
) -> Result<()> {
    let current_version =
        config_db.installed_channels.get(channel).ok_or_else(|| anyhow!("Trying to get the installed version for a channel that does not exist in the config database."))?;

    match current_version {
        JuliaupConfigChannel::SystemChannel { version } => {
            let should_version = version_db.available_channels.get(channel);

            if let Some(should_version) = should_version {
                if &should_version.version != version {
                    install_version(&should_version.version, config_db, version_db, paths)
                        .with_context(|| {
                            format!(
                                "Failed to install '{}' while updating channel '{}'.",
                                should_version.version, channel
                            )
                        })?;

                    config_db.installed_channels.insert(
                        channel.clone(),
                        JuliaupConfigChannel::SystemChannel {
                            version: should_version.version.clone(),
                        },
                    );

                    #[cfg(not(windows))]
                    if config_db.settings.create_channel_symlinks {
                        create_symlink(
                            &JuliaupConfigChannel::SystemChannel {
                                version: should_version.version.clone(),
                            },
                            &format!("julia-{}", channel),
                            paths,
                        )?;
                    }
                }
            } else if ignore_non_updatable_channel {
                eprintln!("Skipping update for '{}' channel, it no longer exists in the version database.", channel);
            } else {
                bail!(
                    "Failed to update '{}' because it no longer exists in the version database.",
                    channel
                );
            }
        }
        JuliaupConfigChannel::LinkedChannel {
            command: _,
            args: _,
        } => {
            if !ignore_non_updatable_channel {
                bail!(
                    "Failed to update '{}' because it is a linked channel.",
                    channel
                );
            } else {
            }
        }
    }

    Ok(())
}

pub fn run_command_update(channel: Option<String>, paths: &GlobalPaths) -> Result<()> {
    update_version_db(paths).with_context(|| "Failed to update versions db.")?;

    let version_db =
        load_versions_db(paths).with_context(|| "`update` command failed to load versions db.")?;

    let mut config_file = load_mut_config_db(paths)
        .with_context(|| "`update` command failed to load configuration data.")?;

    match channel {
        None => {
            for (k, _) in config_file.data.installed_channels.clone() {
                update_channel(&mut config_file.data, &k, &version_db, true, paths)?;
            }
        }
        Some(channel) => {
            if !config_file.data.installed_channels.contains_key(&channel) {
                bail!(
                    "'{}' cannot be updated because it is currently not installed.",
                    channel
                );
            }

            update_channel(&mut config_file.data, &channel, &version_db, false, paths)?;
        }
    };

    garbage_collect_versions(&mut config_file.data, paths)?;

    save_config_db(&mut config_file)
        .with_context(|| "`update` command failed to save configuration db.")?;

    Ok(())
}
