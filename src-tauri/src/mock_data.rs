// mock_data.rs
// Contains mock win data for development

pub struct MockWin<'a> {
    pub date: &'a str,
    pub text: &'a str,
    pub tags: &'a str,
}

pub fn get_mock_wins() -> Vec<MockWin<'static>> {
    let mut wins = vec![
        // --- Comparison Examples ---
        // Should match exactly
        MockWin {
            date: "2025-12-01",
            text: "Jogged in the park for 30 minutes.",
            tags: "jog,park,exercise,health",
        },
        // Should add extra tag (system might add 'positive')
        MockWin {
            date: "2025-12-02",
            text: "Had a wonderful dinner with family.",
            tags: "dinner,family,relationships",
        },
        // Should have missing tag (system might miss 'relationships')
        MockWin {
            date: "2025-12-03",
            text: "Finished a tough project milestone.",
            tags: "project,work,achievement",
        },
        // Should add new tag (system might add 'mental')
        MockWin {
            date: "2025-12-04",
            text: "Meditated for 20 minutes.",
            tags: "meditation,health",
        },
        // Should match exactly
        MockWin {
            date: "2025-12-05",
            text: "Played basketball with friends.",
            tags: "basketball,exercise,sport,friend",
        },
        // Should add extra tag (system might add 'positive')
        MockWin {
            date: "2025-12-06",
            text: "Cooked a healthy meal!",
            tags: "cook,health,food",
        },
        // Should have missing tag (system might miss 'food')
        MockWin {
            date: "2025-12-07",
            text: "Ate fruit for breakfast.",
            tags: "fruit,breakfast,health",
        },
        // Should add new tag (system might add 'rest')
        MockWin {
            date: "2025-12-08",
            text: "Took a nap in the afternoon.",
            tags: "nap,health",
        },
        // Should match exactly
        MockWin {
            date: "2025-12-09",
            text: "Went to the gym.",
            tags: "gym,health,exercise",
        },
        // Should add extra tag (system might add 'positive')
        MockWin {
            date: "2025-12-10",
            text: "Did a group workout session.",
            tags: "group,workout,exercise,health",
        },
        // --- End Comparison Examples ---
        MockWin {
            date: "2025-12-01",
            text: "Absolutely crushed my workout today! 💪",
            tags: "workout,health,exercise,positive",
        },
        MockWin {
            date: "2025-12-02",
            text: "Had an amazing dinner with Alice at Gold's Gym.",
            tags: "",
        },
        MockWin {
            date: "2025-12-03",
            text: "Felt really down after failing the test...",
            tags: "test,school,negative",
        },
        MockWin {
            date: "2025-12-04",
            text: "Terrible day at work.",
            tags: "",
        },
        MockWin {
            date: "2025-12-05",
            text: "Met Bob at Starbucks.",
            tags: "bob,starbucks,positive",
        },
        MockWin {
            date: "2025-12-06",
            text: "Visited Central Park with Sarah!",
            tags: "",
        },
        MockWin {
            date: "2025-12-07",
            text: "Had a great family dinner!",
            tags: "family,relationships,positive",
        },
        MockWin {
            date: "2025-12-08",
            text: "Completed a tough exam.",
            tags: "",
        },
        MockWin {
            date: "2025-12-09",
            text: "Cleaned the house.",
            tags: "clean,life,neutral",
        },
        MockWin {
            date: "2025-12-10",
            text: "Sent important emails.",
            tags: "",
        },
        // Health-focused and varied
        MockWin {
            date: "2025-12-11",
            text: "Ran 5km in the park!",
            tags: "run,health,exercise,positive",
        },
        MockWin {
            date: "2025-12-12",
            text: "Did a yoga session.",
            tags: "",
        },
        MockWin {
            date: "2025-12-13",
            text: "Cooked a healthy meal.",
            tags: "cook,health,life,positive",
        },
        MockWin {
            date: "2025-12-14",
            text: "Meditated for 20 minutes.",
            tags: "",
        },
        MockWin {
            date: "2025-12-15",
            text: "Slept 8 hours!",
            tags: "sleep,health,positive",
        },
        MockWin {
            date: "2025-12-16",
            text: "Went swimming.",
            tags: "",
        },
        MockWin {
            date: "2025-12-17",
            text: "Took vitamins.",
            tags: "vitamins,health,neutral",
        },
        MockWin {
            date: "2025-12-18",
            text: "Had a checkup at the doctor.",
            tags: "",
        },
        MockWin {
            date: "2025-12-19",
            text: "Did 30 pushups!",
            tags: "pushups,health,exercise,positive",
        },
        MockWin {
            date: "2025-12-20",
            text: "Walked 10,000 steps.",
            tags: "",
        },
        MockWin {
            date: "2025-12-21",
            text: "Drank 2L of water.",
            tags: "water,health,neutral",
        },
        MockWin {
            date: "2025-12-22",
            text: "Ate a salad for lunch.",
            tags: "",
        },
        MockWin {
            date: "2025-12-23",
            text: "Did a HIIT workout.",
            tags: "hiit,health,exercise,neutral",
        },
        MockWin {
            date: "2025-12-24",
            text: "Stretched after waking up.",
            tags: "",
        },
        MockWin {
            date: "2025-12-25",
            text: "Took a rest day.",
            tags: "rest,health,neutral",
        },
        MockWin {
            date: "2025-12-26",
            text: "Went hiking.",
            tags: "",
        },
        MockWin {
            date: "2025-12-27",
            text: "Played basketball.",
            tags: "basketball,health,exercise,sport,neutral",
        },
        MockWin {
            date: "2025-12-28",
            text: "Did a cycling session.",
            tags: "",
        },
        MockWin {
            date: "2025-12-29",
            text: "Tried a new healthy recipe!",
            tags: "cook,health,food,positive",
        },
        MockWin {
            date: "2025-12-30",
            text: "Had a relaxing bath.",
            tags: "",
        },
        MockWin {
            date: "2025-12-31",
            text: "Practiced deep breathing.",
            tags: "breathing,health,mental,neutral",
        },
        // Add more varied and health-related wins
    ];
    // Add more to reach 100
    let mut i = 32;
    while wins.len() < 100 {
        let day = 1 + (i % 31);
        let text = match i % 10 {
            0 => format!("Did {} squats", 10 + i),
            1 => "Ate fruit for breakfast".to_string(),
            2 => format!("Jogged for {} minutes", 15 + (i % 20)),
            3 => "Took a nap in the afternoon".to_string(),
            4 => "Played tennis with a friend".to_string(),
            5 => "Did a mindfulness exercise".to_string(),
            6 => "Went to the gym".to_string(),
            7 => "Cooked a vegetarian dinner".to_string(),
            8 => "Did a group workout session".to_string(),
            _ => "Walked in the neighborhood".to_string(),
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
