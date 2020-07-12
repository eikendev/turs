use clap::{App, ArgMatches, SubCommand};
use git2::{Repository, Status, StatusOptions};
use std::env;

use super::defaults;

fn status(r: &Repository) -> Option<String> {
    let mut conflicted = false;
    let mut index_dirty = false;
    let mut wt_dirty = false;
    let mut index_new = false;
    let mut wt_new = false;

    let head = match r.head() {
        Ok(head) => head,
        _ => return None,
    };

    let mut branch;
    branch = head.shorthand().unwrap().to_string();
    branch = format!("%F{{{}}}{}%f", defaults::color::GREY, branch);

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = match r.statuses(Some(&mut opts)) {
        Ok(statuses) => statuses,
        _ => return None,
    };

    let iterator = statuses
        .iter()
        .filter(|e| e.status() != Status::CURRENT && e.status() != Status::IGNORED);

    for entry in iterator {
        let status = entry.status();

        conflicted = match status {
            s if s.is_conflicted() => true,
            _ => conflicted,
        };

        index_dirty = match status {
            s if s.is_index_deleted() => true,
            s if s.is_index_modified() => true,
            s if s.is_index_renamed() => true,
            s if s.is_index_typechange() => true,
            _ => index_dirty,
        };

        wt_dirty = match status {
            s if s.is_wt_deleted() => true,
            s if s.is_wt_modified() => true,
            s if s.is_wt_renamed() => true,
            s if s.is_wt_typechange() => true,
            _ => wt_dirty,
        };

        index_new = match status {
            s if s.is_index_new() => true,
            _ => index_new,
        };

        wt_new = match status {
            s if s.is_wt_new() => true,
            _ => wt_new,
        };

        if conflicted && index_dirty && wt_dirty && index_new && wt_new {
            break;
        }
    }

    let (color, symbol) = if conflicted {
        (defaults::color::RED, defaults::symbol::CONFLICT)
    } else {
        (defaults::color::GREEN, defaults::symbol::NOCONFLICT)
    };

    let status_signs = format!(" %F{{{}}}{}%f", color, symbol);
    branch.push_str(&status_signs);

    if index_dirty {
        let status_signs = format!("%F{{{}}}{}%f", defaults::color::GREEN, defaults::symbol::DIRTY);
        branch.push_str(&status_signs);
    }
    if wt_dirty {
        let status_signs = format!("%F{{{}}}{}%f", defaults::color::YELLOW, defaults::symbol::DIRTY);
        branch.push_str(&status_signs);
    }
    if index_new {
        let status_signs = format!("%F{{{}}}{}%f", defaults::color::GREEN, defaults::symbol::NEW);
        branch.push_str(&status_signs);
    }
    if wt_new {
        let status_signs = format!("%F{{{}}}{}%f", defaults::color::YELLOW, defaults::symbol::NEW);
        branch.push_str(&status_signs);
    }

    Some(branch)
}

pub fn display(_matches: &ArgMatches) {
    let path = match env::current_dir() {
        Ok(path) => path,
        _ => return,
    };

    let display = match Repository::discover(path) {
        Ok(repository) => status(&repository),
        _ => return,
    };

    if let Some(d) = display {
        println!("[{}]", d);
    }
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
    SubCommand::with_name("rprompt")
}
