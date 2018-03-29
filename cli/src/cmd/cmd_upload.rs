use ffsend_api::url::{ParseError, Url};

use rpassword::prompt_password_stderr;
use super::clap::{App, Arg, ArgMatches, SubCommand};

use app::SEND_DEF_HOST;
use util::quit_error_msg;

/// The upload command.
pub struct CmdUpload<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a: 'b, 'b> CmdUpload<'a> {
    /// Build the sub command definition.
    pub fn build<'y, 'z>() -> App<'y, 'z> {
        // Build the subcommand
        #[allow(unused_mut)]
        let mut cmd = SubCommand::with_name("upload")
            .about("Upload files.")
            .visible_alias("u")
            .visible_alias("up")
            .arg(Arg::with_name("FILE")
                .help("The file to upload")
                .required(true)
                .multiple(false))
            .arg(Arg::with_name("name")
                .long("name")
                .short("n")
                .alias("file")
                .alias("f")
                .value_name("NAME")
                .help("Rename the file being uploaded"))
            .arg(Arg::with_name("password")
                .long("password")
                .short("p")
                .alias("pass")
                .value_name("PASSWORD")
                .min_values(0)
                .max_values(1)
                .help("Protect the file with a password"))
            .arg(Arg::with_name("host")
                .long("host")
                .short("h")
                .alias("server")
                .value_name("URL")
                .default_value(SEND_DEF_HOST)
                .help("The Send host to upload to"))
            .arg(Arg::with_name("open")
                .long("open")
                .short("o")
                .help("Open the share link in your browser"));

        // Optional clipboard support
        #[cfg(feature = "clipboard")] {
            cmd = cmd.arg(Arg::with_name("copy")
                .long("copy")
                .short("c")
                .help("Copy the share link to your clipboard"));
        }

        cmd
    }

    /// Parse CLI arguments, from the given parent command matches.
    pub fn parse(parent: &'a ArgMatches<'a>) -> Option<CmdUpload<'a>> {
        parent.subcommand_matches("upload")
            .map(|matches| CmdUpload { matches })
    }

    /// The the name to use for the uploaded file.
    /// If no custom name is given, none is returned.
    // TODO: validate custom names, no path separators
    // TODO: only allow extension renaming with force flag
    pub fn name(&'a self) -> Option<&'a str> {
        // Get the chosen file name
        let name = self.matches.value_of("name")?;

        // The file name must not be empty
        if name.trim().is_empty() {
            // TODO: return an error here
            panic!("the new name must not be empty");
        }

        Some(name)
    }

    /// Get the selected file to upload.
    // TODO: maybe return a file or path instance here
    pub fn file(&'a self) -> &'a str {
        self.matches.value_of("FILE")
            .expect("no file specified to upload")
    }

    /// Get the host to upload to.
    ///
    /// This method parses the host into an `Url`.
    /// If the given host is invalid,
    /// the program will quit with an error message.
    pub fn host(&'a self) -> Url {
        // Get the host
        let host = self.matches.value_of("host")
            .expect("missing host");

        // Parse the URL
        match Url::parse(host) {
            Ok(url) => url,
            Err(ParseError::EmptyHost) =>
                quit_error_msg("Emtpy host given"),
            Err(ParseError::InvalidPort) =>
                quit_error_msg("Invalid host port"),
            Err(ParseError::InvalidIpv4Address) =>
                quit_error_msg("Invalid IPv4 address in host"),
            Err(ParseError::InvalidIpv6Address) =>
                quit_error_msg("Invalid IPv6 address in host"),
            Err(ParseError::InvalidDomainCharacter) =>
                quit_error_msg("Host domains contains an invalid character"),
            Err(ParseError::RelativeUrlWithoutBase) =>
                quit_error_msg("Host domain doesn't contain a host"),
            _ => quit_error_msg("The given host is invalid"),
        }
    }

    /// Check whether to open the file URL in the user's browser.
    pub fn open(&self) -> bool {
        self.matches.is_present("open")
    }

    /// Check whether to copy the file URL in the user's clipboard.
    #[cfg(feature = "clipboard")]
    pub fn copy(&self) -> bool {
        self.matches.is_present("copy")
    }

    /// Get the password.
    /// `None` is returned if no password was specified.
    pub fn password(&'a self) -> Option<String> {
        // Return none if the property was not set
        if !self.matches.is_present("password") {
            return None;
        }

        // Get the password from the arguments
        if let Some(password) = self.matches.value_of("password") {
            return Some(password.into());
        }

        // Prompt for the password
        // TODO: don't unwrap/expect
        // TODO: create utility function for this
        Some(
            prompt_password_stderr("Password: ")
                .expect("failed to read password from stdin")
        )
    }
}
