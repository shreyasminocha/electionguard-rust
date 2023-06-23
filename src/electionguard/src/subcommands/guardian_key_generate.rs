// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

use std::path::PathBuf;

use anyhow::{bail, Result};

use eg::key::GuardianSecretKey;

use crate::{
    artifacts_dir::ArtifactFile, common_utils::load_election_parameters,
    subcommand_helper::SubcommandHelper, subcommands::Subcommand,
};

/// Generate a guardian secret key and public key.
#[derive(clap::Args, Debug, Default)]
pub(crate) struct GuardianSecretKeyGenerate {
    /// Guardian number, 0 <= i < n.
    #[arg(long)]
    i: u16,

    /// Guardian's name or other short description.
    #[arg(long)]
    name: Option<String>,

    /// File to which to write the guardian's secret key.
    /// Default is in the guardian's dir under the artifacts dir.
    /// If "-", write to stdout.
    #[arg(long)]
    secret_key_out_file: Option<PathBuf>,

    /// File to which to write the guardian's public key.
    /// Default is in the artifacts dir.
    /// If "-", write to stdout.
    #[arg(long)]
    public_key_out_file: Option<PathBuf>,
}

impl Subcommand for GuardianSecretKeyGenerate {
    fn uses_csprng(&self) -> bool {
        true
    }

    fn do_it(&mut self, subcommand_helper: &mut SubcommandHelper) -> Result<()> {
        let mut csprng =
            subcommand_helper.get_csprng(format!("GuardianKeyGenerate({})", self.i).as_bytes())?;

        //? TODO: Do we need a command line arg to specify the election parameters source?
        let election_parameters =
            load_election_parameters(&subcommand_helper.artifacts_dir, &mut csprng)?;

        let varying_parameters = &election_parameters.varying_parameters;

        #[allow(clippy::nonminimal_bool)]
        if !(self.i < varying_parameters.n) {
            bail!(
                "Guardian number {} must be less than n = {} from election parameters",
                self.i,
                varying_parameters.n
            );
        }

        let secret_key = GuardianSecretKey::generate(
            &mut csprng,
            &election_parameters,
            self.i,
            self.name.clone(),
        );

        subcommand_helper.artifacts_dir.out_file_write(
            &self.secret_key_out_file,
            ArtifactFile::GuardianSecretKey(self.i),
            format!("secret key for guardian {}", self.i).as_str(),
            secret_key.to_json().as_bytes(),
        )?;

        let public_key = secret_key.make_public_key();

        subcommand_helper.artifacts_dir.out_file_write(
            &self.public_key_out_file,
            ArtifactFile::GuardianPublicKey(self.i),
            format!("public key for guardian {}", self.i).as_str(),
            public_key.to_json().as_bytes(),
        )
    }
}
