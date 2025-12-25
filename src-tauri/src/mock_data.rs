// mock_data.rs
// Contains mock win data for development

pub struct MockWin<'a> {
    pub date: &'a str,
    pub text: &'a str,
    pub tags: &'a str,
}

pub fn get_mock_wins() -> Vec<MockWin<'static>> {
    let mut wins = vec![
        MockWin { date: "2025-12-01", text: "Finished a big project milestone", tags: "project,work" },
        MockWin { date: "2025-12-02", text: "Went for a walk and cleared my head", tags: "walk,health" },
        MockWin { date: "2025-12-03", text: "Helped a friend with homework", tags: "friend,homework,school" },
        MockWin { date: "2025-12-04", text: "Organized my workspace", tags: "admin,organization" },
        MockWin { date: "2025-12-05", text: "Read a new book on learning", tags: "read,learning" },
        MockWin { date: "2025-12-06", text: "Attended a class and took notes", tags: "class,school" },
        MockWin { date: "2025-12-07", text: "Had a great family dinner", tags: "family,relationships" },
        MockWin { date: "2025-12-08", text: "Completed a tough exam", tags: "exam,school" },
        MockWin { date: "2025-12-09", text: "Cleaned the house", tags: "clean,life" },
        MockWin { date: "2025-12-10", text: "Sent important emails", tags: "email,admin" },
        // Health-focused and varied
        MockWin { date: "2025-12-11", text: "Ran 5km in the park", tags: "run,health,exercise" },
        MockWin { date: "2025-12-12", text: "Did a yoga session", tags: "yoga,health,exercise" },
        MockWin { date: "2025-12-13", text: "Cooked a healthy meal", tags: "cook,health,life" },
        MockWin { date: "2025-12-14", text: "Meditated for 20 minutes", tags: "meditation,health,mental" },
        MockWin { date: "2025-12-15", text: "Slept 8 hours", tags: "sleep,health" },
        MockWin { date: "2025-12-16", text: "Went swimming", tags: "swim,health,exercise" },
        MockWin { date: "2025-12-17", text: "Took vitamins", tags: "vitamins,health" },
        MockWin { date: "2025-12-18", text: "Had a checkup at the doctor", tags: "doctor,health" },
        MockWin { date: "2025-12-19", text: "Did 30 pushups", tags: "pushups,health,exercise" },
        MockWin { date: "2025-12-20", text: "Walked 10,000 steps", tags: "walk,health,exercise" },
        MockWin { date: "2025-12-21", text: "Drank 2L of water", tags: "water,health" },
        MockWin { date: "2025-12-22", text: "Ate a salad for lunch", tags: "salad,health,food" },
        MockWin { date: "2025-12-23", text: "Did a HIIT workout", tags: "hiit,health,exercise" },
        MockWin { date: "2025-12-24", text: "Stretched after waking up", tags: "stretch,health,exercise" },
        MockWin { date: "2025-12-25", text: "Took a rest day", tags: "rest,health" },
        MockWin { date: "2025-12-26", text: "Went hiking", tags: "hike,health,exercise,nature" },
        MockWin { date: "2025-12-27", text: "Played basketball", tags: "basketball,health,exercise,sport" },
        MockWin { date: "2025-12-28", text: "Did a cycling session", tags: "cycling,health,exercise" },
        MockWin { date: "2025-12-29", text: "Tried a new healthy recipe", tags: "cook,health,food" },
        MockWin { date: "2025-12-30", text: "Had a relaxing bath", tags: "bath,health,mental" },
        MockWin { date: "2025-12-31", text: "Practiced deep breathing", tags: "breathing,health,mental" },
        // Add more varied and health-related wins
    ];
    // Add more to reach 100
    let mut i = 32;
    while wins.len() < 100 {
        let day = 1 + (i % 31);
        let text = match i % 10 {
            0 => format!("Did {} squats", 10 + i),
            1 => format!("Ate fruit for breakfast"),
            2 => format!("Jogged for {} minutes", 15 + (i % 20)),
            3 => format!("Took a nap in the afternoon"),
            4 => format!("Played tennis with a friend"),
            5 => format!("Did a mindfulness exercise"),
            6 => format!("Went to the gym"),
            7 => format!("Cooked a vegetarian dinner"),
            8 => format!("Did a group workout session"),
            _ => format!("Walked in the neighborhood"),
        };
        let tags = match i % 10 {
            0 => "squats,health,exercise",
            1 => "fruit,health,food",
            2 => "jog,health,exercise",
            3 => "nap,health,rest",
            4 => "tennis,health,exercise,sport",
            5 => "mindfulness,health,mental",
            6 => "gym,health,exercise",
            7 => "vegetarian,health,food",
            8 => "group,health,exercise",
            _ => "walk,health,exercise",
        };
        wins.push(MockWin {
            date: Box::leak(format!("2025-12-{:02}", day).into_boxed_str()),
            text: Box::leak(text.into_boxed_str()),
            tags,
        });
        i += 1;
    }
    wins
}
