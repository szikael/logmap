extern crate logmap;

#[test]
fn no_alts_include_num_no_cols_skipped() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 0;
    log_filters.ignore_numeric_words = false;
    log_filters.ignore_first_columns = 0;

    log_filters.learn_line("Sep 26 09:13:15 anonymous_hostname systemd-logind[572]: Removed session c524.");
    log_filters.learn_line("Sep 27 19:27:53 anonymous_hostname systemd-logind[572]: Removed session c525.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname systemd-logind[572]: Removed session c526.");

    let mut expected: String = "[Sep],[26],[09],[13],[15],[anonymous_hostname],[systemd-logind],[572],[Removed],[session],[c524],".to_string();
                 expected += "\n[Sep],[27],[19],[27],[53],[anonymous_hostname],[systemd-logind],[572],[Removed],[session],[c525],";
                 expected += "\n[Sep],[28],[13],[41],[26],[anonymous_hostname],[systemd-logind],[572],[Removed],[session],[c526]";

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn no_alts_no_nums_no_cols_skipped() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 0;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 0;

    log_filters.learn_line("Sep 26 09:13:15 anonymous_hostname systemd-logind[572]: Removed session c524.");
    log_filters.learn_line("Sep 27 19:27:53 anonymous_hostname systemd-logind[572]: Removed session c525.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname systemd-logind[572]: Removed session c526.");

    let mut expected: String = "[Sep],[anonymous_hostname],[systemd-logind],[Removed],[session],[c524],".to_string();
                 expected += "\n[Sep],[anonymous_hostname],[systemd-logind],[Removed],[session],[c525],";
                 expected += "\n[Sep],[anonymous_hostname],[systemd-logind],[Removed],[session],[c526]";

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn no_alts_no_nums_extended_no_cols_skipped() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 0;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 0;

    log_filters.learn_line(&("Dec 18 09:59:36 host_name [error] 19901#19901: *180073 open() \"/path/to/file\"".to_string() +
        "failed (2: No such file or directory), client: 127.0.0.1, server: some.example.com, request:" +
        "\"GET /request/url HTTP/1.1\", host: \"some.example.com\""));

    let expected: String = "[Dec],[host_name],[error],[open],[path],[to],[file]".to_string() +
        ",[failed],[No],[such],[file],[or],[directory],[client],[server],[some],[example],[com],[request]" +
        ",[GET],[request],[url],[HTTP],[host],[some],[example],[com]";

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn no_alts_no_nums_one_col_skipped() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 0;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 26 09:13:15 anonymous_hostname systemd-logind[572]: Removed session c524.");
    log_filters.learn_line("Sep 27 19:27:53 anonymous_hostname systemd-logind[572]: Removed session c525.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname systemd-logind[572]: Removed session c526.");

    let mut expected: String = "[anonymous_hostname],[systemd-logind],[Removed],[session],[c524],".to_string();
                 expected += "\n[anonymous_hostname],[systemd-logind],[Removed],[session],[c525],";
                 expected += "\n[anonymous_hostname],[systemd-logind],[Removed],[session],[c526]";

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn one_alt_no_nums_one_col_skipped() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 1;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 26 09:13:15 anonymous_hostname systemd-logind[572]: Removed session c524.");
    log_filters.learn_line("Sep 27 19:27:53 anonymous_hostname systemd-logind[572]: Removed session c525.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname systemd-logind[572]: Removed session c526.");

    let expected: String = "[anonymous_hostname],[systemd-logind],[Removed],[session],[c524,c525,c526]".to_string();

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn one_alt_no_nums_one_col_skipped_followed_by_short_line() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 1;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 26 09:13:15 anonymous_hostname systemd-logind[572]: Removed session c524.");
    log_filters.learn_line("Sep 27 19:27:53 anonymous_hostname systemd-logind[572]: Removed session c525.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname systemd-logind[572]: Removed session c526.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname");

    let mut expected: String = "[anonymous_hostname],[systemd-logind],[Removed],[session],[c524,c525,c526],".to_string();
                 expected += "\n[anonymous_hostname]";

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn one_alt_no_nums_one_col_skipped_followed_by_long_line() {
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 1;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname");
    log_filters.learn_line("Sep 26 09:13:15 anonymous_hostname systemd-logind[572]: Removed session c524.");
    log_filters.learn_line("Sep 27 19:27:53 anonymous_hostname systemd-logind[572]: Removed session c525.");
    log_filters.learn_line("Sep 28 13:41:26 anonymous_hostname systemd-logind[572]: Removed session c526.");

    let mut expected: String = "[anonymous_hostname],".to_string();
                 expected += "\n[anonymous_hostname],[systemd-logind],[Removed],[session],[c524,c525,c526]";

    assert_eq!(log_filters.to_string(), expected);
}

#[test]
fn one_alt_no_nums_one_col_skipped_repeated_filter_word_at_the_log_end() {
    // Test without duplicate at the end
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 1;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 22 22:27:52 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir1\")");
    log_filters.learn_line("Sep 22 22:28:40 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir2\")");
    log_filters.learn_line("Sep 22 22:32:22 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir2/dir3\")");

    let expected: String = "[some_hostname],[dolphin],[org],[kde],[dolphin],[slotUrlSelectionRequested],[QUrl],[file],[some],[path],[dir1,dir2],[dir3,.]".to_string();

    assert_eq!(log_filters.to_string(), expected);

    // Test with duplicate at the end
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 1;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 22 22:27:52 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir1\")");
    log_filters.learn_line("Sep 22 22:28:40 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir2\")");
    log_filters.learn_line("Sep 22 22:32:22 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir2/dir1\")");

    let expected: String = "[some_hostname],[dolphin],[org],[kde],[dolphin],[slotUrlSelectionRequested],[QUrl],[file],[some],[path],[dir1,dir2],[dir1,.]".to_string();

    assert_eq!(log_filters.to_string(), expected);

    // Test with two duplicates at the end
    let mut log_filters = logmap::logmap::LogFilters::new();
    log_filters.max_allowed_new_alternatives = 2;
    log_filters.ignore_numeric_words = true;
    log_filters.ignore_first_columns = 1;

    log_filters.learn_line("Sep 22 22:27:52 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir1\")");
    log_filters.learn_line("Sep 22 22:28:40 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir2\")");
    log_filters.learn_line("Sep 22 22:32:22 some_hostname dolphin[7229]: org.kde.dolphin: slotUrlSelectionRequested:  QUrl(\"file:///some/path/dir2/dir1/dir1\")");

    let expected: String = "[some_hostname],[dolphin],[org],[kde],[dolphin],[slotUrlSelectionRequested],[QUrl],[file],[some],[path],[dir1,dir2],[dir1,.],[dir1,.]".to_string();

    assert_eq!(log_filters.to_string(), expected);
}
