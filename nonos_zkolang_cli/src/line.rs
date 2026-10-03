/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A command line: one file, and the flags a command takes, each with its value. */

/** The parsed arguments of one command. */
pub(crate) struct Line<'a> {
    /** The program file. */
    pub(crate) file: &'a str,
    flags: Vec<(&'a str, &'a str)>,
}

impl<'a> Line<'a> {
    /**
     * Split `args` into the file and the flags. A flag must be one of `known`, must be
     * followed by its value, and may be given once; exactly one argument is the file.
     * After `--` every argument is a file, so a file whose name starts with `-` can
     * still be named. A mistake is an error that ends with `usage`.
     */
    pub(crate) fn parse(args: &'a [String], known: &[&str], usage: &str) -> Result<Self, String> {
        Line::parse_with(args, (known, &[]), usage)
    }

    /** As `parse`, with the flags `switches` too, which take no value. */
    pub(crate) fn parse_with(
        args: &'a [String],
        (known, switches): (&[&str], &[&str]),
        usage: &str,
    ) -> Result<Self, String> {
        let wrong = |why: String| format!("{why}\n{usage}");
        let (mut file, mut flags, mut only_files) = (None, Vec::new(), false);
        let mut it = args.iter().map(String::as_str);
        while let Some(a) = it.next() {
            if a == "--" && !only_files {
                only_files = true;
            } else if a.starts_with('-') && !only_files {
                let value = match (known.contains(&a), switches.contains(&a)) {
                    (true, _) => it
                        .next()
                        .ok_or_else(|| wrong(format!("{a} needs a value")))?,
                    (false, true) => "",
                    (false, false) => return Err(wrong(format!("unknown flag {a}"))),
                };
                if flags.iter().any(|&(f, _)| f == a) {
                    return Err(wrong(format!("{a} is given twice")));
                }
                flags.push((a, value));
            } else if file.replace(a).is_some() {
                return Err(wrong(String::from("more than one file given")));
            }
        }
        let file = file.ok_or_else(|| wrong(String::from("no file given")))?;
        Ok(Line { file, flags })
    }

    /** The value given for `flag`, if it was given. */
    pub(crate) fn value(&self, flag: &str) -> Option<&'a str> {
        self.flags
            .iter()
            .find(|&&(f, _)| f == flag)
            .map(|&(_, v)| v)
    }

    /** Whether the switch `flag` was given. */
    pub(crate) fn switch(&self, flag: &str) -> bool {
        self.flags.iter().any(|&(f, _)| f == flag)
    }
}
