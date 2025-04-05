/*
 * Attempts to identify patterns in a Life Grid
 *
 * First approach:
 * - Mark each cell with a group #
 * - Cache groups with bounding boxes from those group numbers
 * - Iter the simulation and if groups collide then merge them
 *
 * Issues:
 * - Still lifes that don't quite touch still affect each other
 *   - Solution A: look 2 cells away for neighbor groups...
 *   - Solution B: somehow detect "neighboring groups" and simulate them together vs not and see if they have the same "behaviour"
 */

use std::collections::HashMap;

use crate::life::{Life, LifeOptions, Pos, pos};

use super::Pattern;

#[derive(Debug)]
struct CellGroup {
    top_left_pos: Pos,
    bot_right_pos: Pos,
}

impl CellGroup {
    pub fn new(pos: Pos) -> Self {
        Self {
            top_left_pos: pos,
            bot_right_pos: pos,
        }
    }

    #[allow(unused)]
    fn within(&self, pos: Pos) -> bool {
        pos.x >= self.top_left_pos.x
            && pos.x <= self.bot_right_pos.x
            && pos.y >= self.top_left_pos.y
            && pos.y <= self.bot_right_pos.y
    }

    fn add(&mut self, pos: Pos) {
        if pos.x < self.top_left_pos.x {
            self.top_left_pos.x = pos.x;
        }
        if pos.y < self.top_left_pos.y {
            self.top_left_pos.y = pos.y;
        }
        if pos.x > self.bot_right_pos.x {
            self.bot_right_pos.x = pos.x;
        }
        if pos.y > self.bot_right_pos.y {
            self.bot_right_pos.y = pos.y;
        }
    }
}

#[derive(Debug)]
struct CellGroupTracker {
    group_grid_map: Vec<Vec<u8>>,
    next_group_id: u8,
    groups: HashMap<u8, CellGroup>,
}

impl CellGroupTracker {
    pub fn new(life: &Life) -> Self {
        let size = life.size();
        let mut tracker = Self {
            group_grid_map: vec![vec![0; size.x as usize]; size.y as usize],
            next_group_id: 1,
            groups: HashMap::new(),
        };

        tracker.setup_tracking(life);

        tracker
    }

    const NEIGHBOR_OFFSETS: &[(i32, i32)] = &[
        // Von-neuman neighborhood
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        // extended neighborhood
        (-2, -2),
        (-1, -2),
        (-0, -2),
        (1, -2),
        (2, -2),
        (2, -1),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 2),
        (0, 2),
        (-1, 2),
        (-2, 2),
        (-2, 1),
        (-2, 0),
        (-2, -1),
    ];

    fn current_group_for_cell(&self, pos: Pos, offset: (i32, i32)) -> u8 {
        let final_pos: (i32, i32) = (pos.x as i32 + offset.0, pos.y as i32 + offset.1);

        // TODO: THIS IS STUPID!
        if final_pos.0 >= 0
            && final_pos.0 < self.group_grid_map[0].len() as i32
            && final_pos.1 >= 0
            && final_pos.1 < self.group_grid_map.len() as i32
        {
            self.group_grid_map[final_pos.1 as usize][final_pos.0 as usize]
        } else {
            0
        }
    }

    fn calc_group_for_cell(&mut self, pos: Pos) -> u8 {
        let mut found_group: Option<u8> = None;
        for neigh_off in Self::NEIGHBOR_OFFSETS {
            let neigh_group = self.current_group_for_cell(pos, *neigh_off);
            if neigh_group > 0 {
                match found_group {
                    Some(group) => {
                        // merge groups :(
                        if neigh_group != group {
                            self.merge_groups(group, neigh_group);
                        }
                    }
                    None => {
                        self.groups.get_mut(&neigh_group).unwrap().add(pos.into());
                        found_group = Some(neigh_group);
                    }
                }
            }
        }

        found_group.unwrap_or_else(|| {
            let group_id = self.next_group_id;
            self.next_group_id += 1;
            self.groups.insert(group_id, CellGroup::new(pos.into()));
            group_id
        })
    }

    fn setup_tracking(&mut self, life: &Life) {
        for (x, y, cell) in life.iter() {
            if cell.is_alive() {
                self.group_grid_map[y as usize][x as usize] = self.calc_group_for_cell(pos(x, y));
            }
        }
    }

    fn update(&mut self, life: &Life) {
        let edge = life.size() - pos(1, 1);
        for (x, y, cell) in life.iter() {
            if cell.is_alive() {
                let group_id = self.calc_group_for_cell(pos(x, y));
                self.group_grid_map[y as usize][x as usize] = group_id;

                // spaceship detection
                if x == 0 || y == 0 || x == edge.x || y == edge.y {
                    // remove this group
                    let group = self.groups.get(&group_id).unwrap();
                    dbg!(group);
                    let area = (group.bot_right_pos - group.top_left_pos) + (1, 1).into();
                    let mut spaceship = Life::new(area.into());
                    for pos in group
                        .top_left_pos
                        .iter(area)
                    {
                        // let this_pos_group = self.group_grid_map[pos.y as usize]
                        //     .get_mut(pos.x as usize)
                        //     .unwrap();

                        if let Some(cell) = life.get_cell(pos.into()) {
                            // OOOOOFFF I don't think this will work
                            if cell.is_alive() { // && this_pos_group == &group_id {
                                // *this_pos_group = 0;
                                spaceship.insert((pos - group.top_left_pos).into(), *cell);
                            }
                        }
                    }

                    let mut pattern = Pattern::new_unclassified(spaceship);
                    pattern.classify();

                    panic!("detected spaceship:\n{}", pattern.to_apgcode());
                    // panic!("")
                }
            }
        }
    }

    fn to_patterns(&self, life: &Life) -> Vec<Pattern> {
        self.groups
            .iter()
            .map(|(_group_id, group_extents)| {
                let size =
                    (group_extents.bot_right_pos - group_extents.top_left_pos) + (1, 1).into();
                let mut new_life = Life::new(size.into());
                for pos in group_extents.top_left_pos.iter(size) {
                    new_life.insert(
                        (pos - group_extents.top_left_pos).into(),
                        *life.get_cell(pos.into()).unwrap(),
                    );
                }
                let mut pattern = Pattern::new_unclassified(new_life);
                pattern.classify();
                pattern
            })
            .collect()
    }

    fn merge_groups(&mut self, master_group: u8, slave_group: u8) {
        // TODO: Only check group bounds
        for y in 0..self.group_grid_map.len() {
            for x in 0..self.group_grid_map[0].len() {
                let val = &mut self.group_grid_map[y][x];
                if *val == slave_group {
                    *val = master_group;
                    self.groups
                        .get_mut(&master_group)
                        .unwrap()
                        .add((x as u16, y as u16).into());
                }
            }
        }

        self.groups.remove(&slave_group);
    }
}

// TODO: PERIOD INPUT!
pub fn identify(life: &Life) -> Vec<Pattern> {
    // TODO: some cases we want big, if we know the pattern fits we don't need to do this...
    let mut test_life = Life::new_ex(life.size() + pos(6, 6), LifeOptions {
        algo: crate::life::LifeAlgoSelect::Cached,
        rule: *life.get_rule(),
    });

    test_life.paste(life, pos(3, 3), None);

    let mut tracker = CellGroupTracker::new(&test_life);
    let start_hash = test_life.hash();

    // TODO: Suspected period input?
    for _ in 0..32 {
        test_life.update();
        tracker.update(&test_life);
        if test_life.hash() == start_hash {
            break;
        }
    }

    tracker.to_patterns(&test_life)
}

#[cfg(test)]
mod identify_tests {
    use crate::life::LifeOptions;

    use super::*;

    #[test]
    fn test_cell_group_tracker() {
        let life = Life::from_plaintext(
            "\
OO..OO
OO..OO",
            LifeOptions::default(),
        );
        let tracker = CellGroupTracker::new(&life);

        assert_eq!(tracker.group_grid_map[1][1], 1);
        assert_eq!(tracker.group_grid_map[1][4], 2);
    }

    #[test]
    #[ignore = "broken"]
    fn test_cell_group_tracker_blinker() {
        let mut life = Life::from_plaintext(
            "\
.O.
.O.
.O.",
            LifeOptions::default(),
        );
        let mut tracker = CellGroupTracker::new(&life);

        assert_eq!(tracker.group_grid_map[1][1], 1);

        life.update();
        tracker.update(&life);

        assert_eq!(tracker.group_grid_map[1][0], 1);
    }

    #[test]
    fn test_identify_block() {
        let mut life = Life::from_plaintext(
            "\
OO..OO
OO..OO",
            LifeOptions::default(),
        );

        let patterns = identify(&mut life);

        assert_eq!(patterns[0].to_apgcode(), "xs4_33");
        assert_eq!(patterns[1].to_apgcode(), "xs4_33");
    }

    #[test]
    #[ignore = "not deterministic map odering :("]
    fn test_identify_block_blink() {
        let mut life = Life::from_plaintext(
            "\
OO...O
OO...O
.....O",
            LifeOptions::default(),
        );

        let patterns = identify(&mut life);

        assert_eq!(patterns[1].to_apgcode(), "xp2_7");
        assert_eq!(patterns[0].to_apgcode(), "xs4_33");
    }

    // https://conwaylife.com/wiki/List_of_common_still_lifes
    #[rustfmt::skip]
    const STILL_LIFES: &[(&str, &str)] = &[
// rust-fmt off
//#     Image	        Pattern	# of cells	Synth. cost	Relative frequency	FC	Apgcode
(/* 1	Block.png */	"Block", /* 	4	2	1 in 2.0141	0.0	*/ "xs4_33"),
(/* 2	Beehive.png */	"Beehive", /* 	6	2	1 in 3.8007	0.9	*/ "xs6_696"),
(/* 3	Loaf.png */	"Loaf", /* 	7	2	1 in 12.883	2.7	*/ "xs7_2596"),
(/* 4	Boat.png */	"Boat", /*	5	2	1 in 13.963	2.8	*/ "xs5_253"),
(/* 5	Ship.png */	"Ship", /*	6	3	1 in 20.223	3.3	*/ "xs6_356"),
(/* 6	Tub.png */	"Tub", /*	4	3	1 in 63.991	5.0	*/ "xs4_252"),
(/* 7	Pond.png */	"Pond", /*	8	2	1 in 65.793	5.0	*/ "xs8_6996"),
(/* 8	Longboat.png */	"Long_boat", /* 7	3	1 in 198.66	6. 6 */ "xs7_25ac"),
(/* 9	Shiptie.png */	"Ship", /*-tie	12	4	1 in 391.58	7. 6*/ "xs12_g8o653z11"),
(/* 10	Barge.png */	"Barge", /*	6	3	1 in 945.81	8.9	*/ "xs6_25a4"),
(/* 11	Halfbakery.png */	"Half-bakery", /*  14	3	1 in 1156.2	9. 2 */ "xs14_g88m952z121"),
(/* 12	Mango.png */	"Mango", /*	8	3	1 in 2567.6	10.3	*/ "xs8_69ic"),
(/* 13	Eater1.png */	"Eater_1", /*	7	2	1 in 3986.2	11.0	*/ "xs7_178c"),
(/* 14	Longbarge.png */	"Long_barge", /*	8	3	1 in 7075.9	11.8	*/ "xs8_25ak8"),
(/* 15	Aircraftcarrier.png */	"Aircraft_carrier", /*	6	4	1 in 8189.3	12.0	*/ "xs6_39c"),
(/* 16	Paperclip.png	*/ "Paperclip	", /* 14	3	1 in 14104	12.8	*/ "xs14_69bqic"),
(/* 17	Longship.png	*/ "Long_ship	", /* 8	4	1 in 20723	13.3	*/ "xs8_35ac"),
(/* 18	Integralsign.png	*/ "Integral_sign	", /* 9	4	1 in 22515	13.4	*/ "xs9_31ego"),
(/* 19	Shillelagh.png	*/ "Shillelagh	", /* 8	4	1 in 26104	13.7	*/ "xs8_3pm"),
(/* 20	Boattie.png	*/ "Boat-tie	", /* 10	4	1 in 26648	13.7	*/ "xs10_g8o652z01"),
(/* 21	Snake.png	*/ "Snake	", /* 6	4	1 in 29798	13.9	*/ "xs6_bd"),
(/* 22	Bigs.png	*/ "Big_S	", /* 14	4	1 in 29826	13.9	*/ "xs14_g88b96z123"),
(/* 23	Bipond.png	*/ "Bi-pond	", /* 16	3	1 in 34255	14.1	*/ "xs16_g88m996z1221"),
(/* 24	Transboatwithtail.png	*/ "Trans-boat_with_tail	", /* 9	4	1 in 62327	14.9	*/ "xs9_178ko"),
(/* 25	Boattieship.png	*/ "Boat_tie_ship	", /* 11	4	1 in 75372	15.2	*/ "xs11_g8o652z11"),
(/* 26	Hat.png	*/ "Hat	", /* 9	3	1 in 79211	15.3	*/ "xs9_4aar"),
(/* 27	Verylongship.png	*/ "Very_long_ship	", /* 10	4	1 in 97166	15.6	*/ "xs10_35ako"),
(/* 28	Verylongboat.png	*/ "Very_long_boat	", /* 9	3	1 in 108510	15.7	*/ "xs9_25ako"),
(/* 29	Tubwithtail.png	*/ "Tub_with_tail	", /* 8	4	1 in 114959	15.8	*/ "xs8_178k8"),
(/* 30	Mirroredtable.png	*/ "Mirrored_table	", /* 12	4	1 in 153578	16.2	*/ "xs12_raar"),
(/* 31	Deadsparkcoil.png	*/ "Dead_spark_coil	", /* 18	4	1 in 162635	16.3	*/ "xs18_rhe0ehr"),
(/* 32	Canoe.png	*/ "Canoe	", /* 8	4	1 in 185277	16.5	*/ "xs8_312ko"),
(/* 33	Beehiveondock.png	*/ "Beehive_on_dock	", /* 16	4	1 in 246355	16.9	*/ "xs16_j1u0696z11"),
(/* 34	Cismirroredbun.png	*/ "Cis-mirrored_bun	", /* 14	4	1 in 251034	16.9	*/ "xs14_6970796"),
(/* 35	Mooseantlers.png	*/ "Moose_antlers	", /* 15	4	1 in 279678	17.1	*/ "xs15_354cgc453"),
(/* 36	Blockontable.png	*/ "Block_on_table	", /* 10	4	1 in 343458	17.4	*/ "xs10_32qr"),
(/* 37	Blockondock.png	*/ "Block_on_dock	", /* 14	4	1 in 377550	17.5	*/ "xs14_j1u066z11"),
(/* 38	Scorpion.png	*/ "Scorpion	", /* 16	4	1 in 469649	17.8	*/ "xs16_69egmiczx1"),
(/* 39	Beehivewithtail.png	*/ "Beehive_with_tail	", /* 10	4	1 in 568015	18.1	*/ "xs10_178kk8"),
(/* 40	Twinhat.png	*/ "Twin_hat	", /* 17	4	1 in 596223	18.2	*/ "xs17_2ege1ege2"),
(/* 41	Loop.png	*/ "Loop	", /* 10	4	1 in 693975	18.4	*/ "xs10_69ar"),
(/* 42	Longsnake.png	*/ "Long_snake	", /* 7	4	1 in 704041	18.4	*/ "xs7_3lo"),
(/* 43	Fourteener.png	*/ "Fourteener	", /* 14	4	1 in 715172	18.4	*/ "xs14_69bo8a6"),
(/* 44	Cismirroredbookend.png	*/ "Cis-mirrored_bookend	", /* 14	4	1 in 875560	18.7	*/ "xs14_39e0e93"),
(/* 45	Cisboatwithtail.png	*/ "Cis-boat_with_tail	", /* 9	4	1 in 1084160	19.0	*/ "xs9_178kc"),
(/* 46	Cisrotatedbookend.png	*/ "Cis-rotated_bookend	", /* 14	4	1 in 1101400	19.1	*/ "xs14_6is079c"),
(/* 47	Elevener.png	*/ "Elevener	", /* 11	4	1 in 1127700	19.1	*/ "xs11_g0s453z11"),
(/* 48	Mirroreddock.png	*/ "Mirrored_dock	", /* 20	4	1 in 1223280	19.2	*/ "xs20_3lkkl3z32w23"),
(/* 49	Blockoncap.png	*/ "Block_on_cap	", /* 12	4	1 in 1266110	19.3	*/ "xs12_330f96"),
(/* 50	Transloafwithtail.png	*/ "Trans-loaf_with_tail	", /* 11	4	1 in 1301200	19.3	*/ "xs11_ggm952z1"),
(/* 51	Cisshillelagh.png	*/ "Cis-shillelagh	", /* 10	4	1 in 1321140	19.3	*/ "xs10_358gkc"),
(/* 52	Transmirroredbun.png	*/ "Trans-mirrored_bun	", /* 14	4	1 in 1361100	19.4	*/ "xs14_69e0eic"),
(/* 53	Transblockonlongbookend.png	*/ "Trans-block_on_long_bookend	", /* 12	4	1 in 1450520	19.5	*/ "xs12_330fho"),
(/* 54	Tubwithnine.png	*/ "Tub_with_nine	", /* 10	4	1 in 1608160	19.6	*/ "xs10_g0s252z11"),
(/* 55	Brokensnake.png	*/ "Broken_snake	", /* 10	4	1 in 1778440	19.8	*/ "xs10_0drz32"),
(/* 56	Transbookendandbun.png	*/ "Trans-bookend_and_bun	", /* 14	4	1 in 1923670	19.9	*/ "xs14_39e0eic"),
(/* 57	Eaterheadsiameseeatertail.png	*/ "Eater_head_siamese_eater_tail	", /* 12	4	1 in 1973390	19.9	*/ "xs12_178c453"),
(/* 58	Blockoncover.png	*/ "Block_on_cover	", /* 12	4	1 in 1975910	19.9	*/ "xs12_178br"),
(/* 59	Cisboatondock.png	*/ "Cis-boat_on_dock	", /* 15	4	1 in 1981540	19.9	*/ "xs15_j1u06a4z11"),
(/* 60	Cisblockonlongbookend.png	*/ "Cis-block_on_long_bookend	", /* 12	4	1 in 2112590	20.0	*/ "xs12_3hu066"),
(/* 61	Verylongsnake.png	*/ "Very_long_snake	", /* 8	4	1 in 2133300	20.0	*/ "xs8_31248c"),
(/* 62	Boatwithlongtail.png	*/ "Boat_with_long_tail	", /* 10	4	1 in 2295220	20.1	*/ "xs10_3215ac"),
(/* 63	Longshillelagh.png	*/ "Long_shillelagh	", /* 9	4	1 in 2387910	20.2	*/ "xs9_312453"),
(/* 64	Beehiveatloaf.png	*/ "Beehive_at_loaf	", /* 13	4	1 in 2404340	20.2	*/ "xs13_g88m96z121"),
(/* 65	Transbunandwing.png	*/ "Trans-bun_and_wing	", /* 15	4	1 in 2865880	20.4	*/ "xs15_259e0eic"),
(/* 66	Longintegral.png	*/ "Long_integral	", /* 10	4	1 in 2981120	20.5	*/ "xs10_3542ac"),
(/* 67	Tubwithlongtail.png	*/ "Tub_with_long_tail	", /* 9	4	1 in 2995220	20.5	*/ "xs9_25a84c"),
(/* 68	Cisbookendandbun.png	*/ "Cis-bookend_and_bun	", /* 14	4	1 in 2999830	20.5	*/ "xs14_39e0e96"),
(/* 69	Hookwithtail.png	*/ "Hook_with_tail	", /* 8	4	1 in 3006130	20.5	*/ "xs8_32qk"),
(/* 70	Loafsiameseloaf.png	*/ "Loaf_siamese_loaf	", /* 11	4	1 in 3068890	20.5	*/ "xs11_69lic"),
(/* 71	Longcanoe.png	*/ "Long_canoe	", /* 9	4	1 in 3742850	20.8	*/ "xs9_g0g853z11"),
(/* 72	Elevenloop.png	*/ "Eleven_loop	", /* 11	4	1 in 3800850	20.8	*/ "xs11_178jd"),
(/* 73	Cisloafontable.png	*/ "Cis-loaf_on_table	", /* 13	4	1 in 3810750	20.9	*/ "xs13_4a960ui"),
(/* 74	Cisloafwithtail.png	*/ "Cis-loaf_with_tail	", /* 11	4	1 in 3828430	20.9	*/ "xs11_178kic"),
(/* 75	Symmetricscorpion.png	*/ "Symmetric_scorpion	", /* 16	4	1 in 3828900	20.9	*/ "xs16_69bob96"),
(/* 76	Clawwithtail.png	*/ "Claw_with_tail	", /* 10	4	1 in 3988780	20.9	*/ "xs10_1784ko"),
(/* 77	Beehat.png	*/ "Bee ", /* hat	15	4	1 in 4202710	21.0	*/ "xs15_3lkm96z01"),
(/* 78	Cismirroreddove.png	*/ "Cis-mirrored_dove	", /* 18	4	1 in 4268330	21.0	*/ "xs18_69is0si96"),
(/* 79	Transrotatedbun.png	*/ "Trans-rotated_bun	", /* 14	4	1 in 4284510	21.0	*/ "xs14_g8o0e96z121"),
(/* 80	Cismirroredwing.png	*/ "Cis-mirrored_wing	", /* 16	4	1 in 4378330	21.1	*/ "xs16_259e0e952"),
(/* 81	Transsnakeonbun.png	*/ "Trans-snake_on_bun	", /* 13	4	1 in 4786430	21.2	*/ "xs13_69e0mq"),
(/* 82	Boattieeatertail.png	*/ "Boat_tie_eater_tail	", /* 12	4	1 in 5090380	21.3	*/ "xs12_256o8a6"),
(/* 83	Snorkelloop.png	*/ "Snorkel_loop	", /* 12	4	1 in 5322100	21.3	*/ "xs12_2egm93"),
(/* 84	Beehiveontable.png	*/ "Beehive_on_table	", /* 12	4	1 in 5381190	21.3	*/ "xs12_6960ui"),
(/* 85	Cisboatontable.png	*/ "Cis-boat_on_table	", /* 11	4	1 in 5521390	21.4	*/ "xs11_2530f9"),
(/* 86	Transbargewithtail.png	*/ "Trans-barge_with_tail	", /* 10	4	1 in 5937700	21.5	*/ "xs10_ggka52z1"),
(/* 87	Transboatondock.png	*/ "Trans-boat_on_dock	", /* 15	4	1 in 6327380	21.6	*/ "xs15_3lk453z121"),
(/* 88	Beehiveoncap.png	*/ "Beehive_on_cap	", /* 14	4	1 in 6410870	21.6	*/ "xs14_6960uic"),
(/* 89	Beehiveatbeehive.png	*/ "Beehive_at_beehive	", /* 12	4	1 in 6424120	21.6	*/ "xs12_o4q552z01"),
(/* 90	Longboattieship.png	*/ "Long_boat_tie_ship	", /* 13	5	1 in 6464860	21.6	*/ "xs13_0g8o653z121"),
(/* 91	Cisboatoncap.png	*/ "Cis-boat_on_cap	", /* 13	4	1 in 6697470	21.7	*/ "xs13_2530f96"),
(/* 92	Beehivewithhookedtail.png	*/ "Beehive_with_hooked_tail	", /* 12	4	1 in 6724400	21.7	*/ "xs12_2egm96"),
(/* 93	Transboatwithnine.png	*/ "Trans-boat_with_nine	", /* 11	4	1 in 6853510	21.7	*/ "xs11_g0s253z11"),
(/* 94	15bentpaperclip.png	*/ "15-bent-paperclip	", /* 15	4	1 in 7054270	21.7	*/ "xs15_4a9raic"),
(/* 95	Beehivewithnine.png	*/ "Beehive_with_nine	", /* 12	4	1 in 7945320	21.9	*/ "xs12_0ggm96z32"),
(/* 96	Verylongbarge.png	*/ "Very_long_barge	", /* 10	5	1 in 8193190	22.0	*/ "xs10_g8ka52z01"),
(/* 97	Translongboatwithtail.png	*/ "Trans-long_boat_with_tail	", /* 11	4	1 in 8468630	22.0	*/ "xs11_ggka53z1"),
(/* 98	Cisbunandwing.png	*/ "Cis-bun_and_wing	", /* 15	4	1 in 9079750	22.1	*/ "xs15_259e0e96"),
(/* 99	Hungryhat.png	*/ "Hungry_hat	", /* 11	4	1 in 9519300	22.2	*/ "xs11_2ege13"),
(/* 100	Transboatontable.png	*/ "Trans-boat_on_table	", /* 11	4	1 in 9854600	22.2	*/ "xs11_2560ui"),
(/* 101	Cisboatwithnine.png	*/ "Cis-boat_with_nine	", /* 11	5	1 in 9906700	22.2	*/ "xs11_g0s256z11"),
(/* 102	Tapeworm.png	*/ "Tapeworm	", /* 13	4	1 in 10359200	22.3	*/ "xs13_321fgkc"),
(/* 103	Loafbacktieloaf.png	*/ "Loaf ", /* back_tie_loaf	14	4	1 in 11036200	22.4	*/ "xs14_69la4ozx11"),
(/* 104	Loaftieeaterwithtail.png	*/ "Loaf_tie_eater_with_tail	", /* 18	4	1 in 11220700	22.4	*/ "xs18_2egm9a4zx346"),
(/* 105	Mangowithblockondock.png	*/ "Mango_with_block_on_dock	", /* 19	4	1 in 11659200	22.5	*/ "xs19_69icw8ozxdd11"),
(/* 106	Transbunanddove.png	*/ "Trans-bun_and_dove	", /* 16	4	1 in 12113700	22.5	*/ "xs16_0ca952z2553"),
(/* 107	Rotatedhouse.png	*/ "Rotated_house	", /* 18	4	1 in 12488600	22.6	*/ "xs18_c4o0ehrz321"), // NOTE: BUG IN WIKI, MISSING TRAILING '1'
(/* 108	Boattielongboat.png	*/ "Boat_tie_long_boat	", /* 12	4	1 in 12526300	22.6	*/ "xs12_0g8o652z121"),
(/* 109	Cismirroredlongbookend.png	*/ "Cis-mirrored_long_bookend	", /* 16	4	1 in 12641500	22.6	*/ "xs16_3hu0uh3"),
(/* 110	Xs19 */ "69bo7pic.png	", /* xs19_69bo7pic	19	4	1 in 13131300	22.6	*/ "xs19_69bo7pic"),
(/* 111	Transshipondock.png	*/ "Trans-ship_on_dock	", /* 16	4	1 in 13274500	22.7	*/ "xs16_3lk453z321"),
(/* 112	Transmangowithtail.png	*/ "Trans-mango_with_tail	", /* 12	4	1 in 13572700	22.7	*/ "xs12_69iczx113"),
(/* 113	Boatwithhookednine.png	*/ "Boat_with_hooked_nine	", /* 13	4	1 in 13914500	22.7	*/ "xs13_31egma4"),
(/* 114	Biloaf2.png	*/ "Bi-loaf ", /* 2	14	4	1 in 14136800	22.7	*/ "xs14_4a9m88gzx121"),
(/* 115	Transbunbridgeloaf.png	*/ "Trans-bun_bridge_loaf	", /* 14	4	1 in 14365100	22.8	*/ "xs14_g8o69a4z121"),
(/* 116	Clawatclaw.png	*/ "Claw_at_claw	", /* 12	5	1 in 14437900	22.8	*/ "xs12_g4q453z11"),
(/* 117	Xs15 */ "0gilicz346.png	", /* Big_S_with_tub	15	4	1 in 14580200	22.8	*/ "xs15_0gilicz346"),
(/* 118	Xs17 */ "2ege1t6zx11.png	", /* Paperclip_with_tail	17	4	1 in 15077300	22.8	*/ "xs17_2ege1t6zx11"),
(/* 119	Crinklyheptominowithhookedtail.png	*/ "Crinkly_heptomino_with_hooked_tail	", /* 13	4	1 in 15084800	22.8	*/ "xs13_32qb96"),
(/* 120	Beehivebridgebun.png	*/ "Beehive_bridge_bun	", /* 13	4	1 in 15816800	22.9	*/ "xs13_g8ge96z121"),
(/* 121	Xs15 */ "259e0e93.png	", /* Cis-bookend_and_wing	15	4	1 in 16005400	22.9	*/ "xs15_259e0e93"),
(/* 122	Boatwithhookedtail.png	*/ "Boat_with_hooked_tail	", /* 11	4	1 in 16719100	23.0	*/ "xs11_178b52"),
(/* 123	Metadockandlongbookend.png	*/ "Meta-dock_and_long_bookend	", /* 18	5	1 in 16939300	23.0	*/ "xs18_j1u0uh3z11"),
(/* 124	0ggca96z3443.png	*/ "Pond_bridge_wing	", /* 16	4	1 in 17112200	23.0	*/ "xs16_0ggca96z3443"),
(/* 125	Carriersiamesecarrier.png	*/ "Carrier_siamese_carrier	", /* 10	4	1 in 17158700	23.0	*/ "xs10_0cp3z32"),
(/* 126	69960uic.png	*/ "Pond_on_cap	", /* 16	4	1 in 17328500	23.0	*/ "xs16_69960uic"),
(/* 127	Verylongshillelagh.png	*/ "Very_long_shillelagh	", /* 10	4	1 in 17338700	23.0	*/ "xs10_0j96z32"),
(/* 128	Housesiameseshillelagh.png	*/ "House_siamese_shillelagh	", /* 13	5	1 in 17464000	23.0	*/ "xs13_354djo"),
(/* 129	Longhookwithtail.png	*/ "Long_hook_with_tail	", /* 9	5	1 in 17539800	23.1	*/ "xs9_178426"),
(/* 130	0mmge96z1221.png	*/ "Block_on_bun_tie_bun	", /* 18	5	1 in 17613200	23.1	*/ "xs18_0mmge96z1221"),
(/* 131	Snakewithfeather.png	*/ "Snake_with_feather	", /* 11	4	1 in 17634100	23.1	*/ "xs11_3586246"),
(/* 132	259e0eio.png	*/ "Trans-bookend_and_wing	", /* 15	4	1 in 18091600	23.1	*/ "xs15_259e0eio"),
(/* 133	Carrierwithfeather.png	*/ "Carrier_with_feather	", /* 11	4	1 in 18408300	23.1	*/ "xs11_31461ac"),
(/* 134	Bargesiameseloaf.png	*/ "Barge_siamese_loaf	", /* 10	5	1 in 18669300	23.1	*/ "xs10_4al96"),
(/* 135	Carriersiamesedock.png	*/ "Carrier_siamese_dock	", /* 14	4	1 in 18707000	23.1	*/ "xs14_j1u413z11"),
(/* 136	Shiftmirroredbookend.png	*/ "Shift-mirrored_bookend	", /* 14	4	1 in 18770600	23.2	*/ "xs14_g4s079cz11"),
(/* 137	Long3boat.png	*/ "Long^3_boat	", /* 11	4	1 in 18795300	23.2	*/ "xs11_g8ka52z11"),
(/* 138	Cismirroredworm.png	*/ "Cis-mirrored_worm	", /* 22	4	1 in 19050100	23.2	*/ "xs22_69b88cz69d113"),
(/* 139	3hu0696.png	*/ "Cis-beehive_on_long_bookend	", /* 14	4	1 in 19600300	23.2	*/ "xs14_3hu0696"),
(/* 140	g8id96z121.png	*/ "Teardrop_with_beehive	", /* 14	5	1 in 20776100	23.3	*/ "xs14_g8id96z121"),
(/* 141	08o0u93zoif032.png	*/ "Trans-rotated_bookend_siamese_table	", /* 22	5	1 in 20952400	23.3	*/ "xs22_08o0u93zoif032"),
(/* 142	Cisbargewithtail.png	*/ "Cis-barge_with_tail	", /* 10	4	1 in 21279400	23.3	*/ "xs10_178ka4"),
(/* 143	Krake.png	*/ "Krake	", /* 13	4	1 in 22200800	23.4	*/ "xs13_31ege13"),
(/* 144	Shiptiesnake.png	*/ "Ship_tie_snake	", /* 12	5	1 in 22201000	23.4	*/ "xs12_3123cko"),
(/* 145	Beehivewithhookednine.png	*/ "Beehive_with_hooked_nine	", /* 14	4	1 in 22264100	23.4	*/ "xs14_31egm96"),
(/* 146	Pondondock.png	*/ "Pond_on_dock	", /* 18	4	1 in 23343300	23.5	*/ "xs18_3lk453z3443"),
(/* 147	Twelveloop.png	*/ "Twelve_loop	", /* 12	4	1 in 23712700	23.5	*/ "xs12_31egma"),
(/* 148	Bookendsiameseshillelagh.png	*/ "Bookend_siamese_shillelagh	", /* 13	4	1 in 24174800	23.5	*/ "xs13_354mp3"),
(/* 149	Oquadloaf.png	*/ "O_quad-loaf	", /* 28	6	1 in 24989800	23.6	*/ "xs28_0g8ka9m88gz122dia521"),
(/* 150	Sidewalk.png	*/ "Sidewalk	", /* 14	4	1 in 25103500	23.6	*/ "xs14_1no3tg"),
(/* 151	Xs14_2egu156.png	*/ "xs14_2egu156	", /* 14	4	1 in 25939500	23.6	*/ "xs14_2egu156"),
(/* 152	Clawatloaf.png	*/ "Claw_at_loaf	", /* 13	4	1 in 26421500	23.6	*/ "xs13_08ka96z321"),
(/* 153	Xs16_259aczx6513.png	*/ "Para-bookend_and_dove	", /* 16	4	1 in 26963100	23.7	*/ "xs16_259aczx6513"),
(/* 154	Metabookendandhouse.png	*/ "Meta-bookend_and_house	", /* 16	4	1 in 27755600	23.7	*/ "xs16_39e0ehr"),
(/* 155	Boattieshillelaghhead.png	*/ "Boat_tie_shillelagh_head	", /* 13	4	1 in 28004500	23.7	*/ "xs13_djozx352"),
(/* 156	Xs16_69is0si6.png	*/ "Cis-bookend_and_dove	", /* 16	4	1 in 28191000	23.7	*/ "xs16_69is0si6"),
(/* 157	Doubleclaw.png	*/ "Double_claw	", /* 12	5	1 in 28727800	23.8	*/ "xs12_651i4ozx11"),
(/* 158	Xs18_8ehlmzw12452.png	*/ "unnamed	", /* 18	4	1 in 29522500	23.8	*/ "xs18_8ehlmzw12452"),
(/* 159	Parabookendandbun.png	*/ "Para-bookend_and_bun	", /* 14	4	1 in 29746600	23.8	*/ "xs14_o4s079cz01"),
(/* 160	Xs15_09v0ccz321.png	*/ "Hook_join_table_and_block	", /* 15	5	1 in 31020600	23.9	*/ "xs15_09v0ccz321"),
(/* 161	Tableanddock.png	*/ "Table_and_dock	", /* 16	5	1 in 31267400	23.9	*/ "xs16_j1u0uiz11"),
(/* 162	Xs15_o4s3pmz01.png	*/ "Bun_on_shillelagh	", /* 15	4	1 in 31546900	23.9	*/ "xs15_o4s3pmz01"),
(/* 163	Long3barge.png	*/ "Long^3_barge	", /* 12	4	1 in 32192500	23.9	*/ "xs12_0g8ka52z121"),
(/* 164	Transloafwithnine.png	*/ "Trans-loaf_with_nine	", /* 13	4	1 in 32324800	23.9	*/ "xs13_0ggm952z32"),
(/* 165	Xs10_31eg8o.png	*/ "Hooked_integral	", /* 10	5	1 in 32377000	23.9	*/ "xs10_31eg8o"),
(/* 166	Clawwithnine.png	*/ "Claw_with_nine	", /* 12	5	1 in 33650900	24.0	*/ "xs12_0ggm93z32"),
(/* 167	Eaterbridgebeehive.png	*/ "Eater_bridge_beehive	", /* 13	5	1 in 33741000	24.0	*/ "xs13_255q8a6"),
(/* 168	Eaterwithcape.png	*/ "Eater_with_cape	", /* 11	5	1 in 34059400	24.0	*/ "xs11_31e853"),
(/* 169	Boattieeaterhead.png	*/ "Boat_tie_eater_head	", /* 12	5	1 in 34150700	24.0	*/ "xs12_0g8o652z23"),
(/* 170	Xs17_3lk453z1243.png	*/ "Ortho-loaf_and_dock	", /* 17	4	1 in 35059600	24.1	*/ "xs17_3lk453z1243"),
(/* 171	Cisboatuponlongbookend.png	*/ "Cis-boat_up_on_long_bookend	", /* 13	4	1 in 35424200	24.1	*/ "xs13_3hu06a4"),
(/* 172	Carriersiameseeaterhead.png	*/ "Carrier_siamese_eater_head	", /* 11	5	1 in 35736300	24.1	*/ "xs11_178c4go"),
(/* 173	Cisshipondock.png	*/ "Cis-ship_and_dock	", /* 16	4	1 in 35993300	24.1	*/ "xs16_j1u06acz11"),
(/* 174	Xs15_25960uh3.png	*/ "Para-loaf_and_long_bookend	", /* 15	4	1 in 36506300	24.1	*/ "xs15_25960uh3"),
(/* 175	Xs15_6t1egoz11.png	*/ "unnamed	", /* 15	4	1 in 37024900	24.1	*/ "xs15_6t1egoz11"),
(/* 176	Cisbunanddove.png	*/ "Cis-bun_and_dove	", /* 16	5	1 in 37518600	24.2	*/ "xs16_697079ic"),
(/* 177	Xs15_g8o6996z121.png	*/ "Bun_on_pond	", /* 15	4	1 in 37949400	24.2	*/ "xs15_g8o6996z121"),
(/* 178	Longclawatlongclaw.png	*/ "Long_claw_at_long_claw	", /* 14	5	1 in 38332600	24.2	*/ "xs14_i5q453z11"),
(/* 179	Brokenmooseantlers.png	*/ "Broken_moose_antlers	", /* 13	4	1 in 39807200	24.2	*/ "xs13_4aarzx123"),
(/* 180	Eatertailsiamesesnake.png	*/ "Eater_tail_siamese_snake	", /* 11	4	1 in 39832000	24.2	*/ "xs11_32132ac"),
(/* 181	Xs17_3lk453z3421.png	*/ "Para-loaf_and_dock	", /* 17	4	1 in 41015300	24.3	*/ "xs17_3lk453z3421"),
(/* 182	Cisboatdownonlongbookend.png	*/ "Cis-boat_down_on_long_bookend	", /* 13	4	1 in 41375100	24.3	*/ "xs13_2530fho"),
(/* 183	Xs15_3213ob96.png	*/ "unnamed	", /* 15	5	1 in 42617300	24.3	*/ "xs15_3213ob96"),
(/* 184	Xs15_08o6996z321.png	*/ "Bookend_on_pond	", /* 15	4	1 in 42969500	24.3	*/ "xs15_08o6996z321"),
(/* 185	Transmirroredbookend.png	*/ "Trans-mirrored_bookend	", /* 14	5	1 in 43083800	24.4	*/ "xs14_39e0eio"),
(/* 186	Xs14_g88q552z121.png	*/ "Wing_on_beehive	", /* 14	5	1 in 43190200	24.4	*/ "xs14_g88q552z121"),
(/* 187	Xs14_31ego8a6.png	*/ "Integral_siamese_eater	", /* 14	4	1 in 43304100	24.4	*/ "xs14_31ego8a6"),
(/* 188	Transloafontable.png	*/ "Trans-loaf_on_table	", /* 13	4	1 in 43660900	24.4	*/ "xs13_25960ui"),
(/* 189	Xs15_69aczw6513.png	*/ "Para-bookend_and_wing	", /* 15	4	1 in 44086600	24.4	*/ "xs15_69aczw6513"),
(/* 190	Verylongcanoe.png	*/ "Very_long_canoe	", /* 10	4	1 in 45254100	24.4	*/ "xs10_xg853z321"),
(/* 191	Long3shillelagh.png	*/ "Long^3_shillelagh	", /* 11	5	1 in 45390600	24.4	*/ "xs11_69jzx56"),
(/* 192	Xs14_g2u0696z11.png	*/ "Trans-beehive_and_long_bookend	", /* 14	5	1 in 45783300	24.4	*/ "xs14_g2u0696z11"),
(/* 193	Xs14_o4id1e8z01.png	*/ "?14-mango_with_bend_tail	", /* 14	4	1 in 46160000	24.4	*/ "xs14_o4id1e8z01"),
(/* 194	Shiptielongship.png	*/ "Ship_tie_long_ship	", /* 14	5	1 in 47135000	24.5	*/ "xs14_0g8o653z321"),
(/* 195	Xs15_69bojd.png	*/ "unnamed	", /* 15	4	1 in 48293300	24.5	*/ "xs15_69bojd"),
(/* 196	Carriersiameseeatertail.png	*/ "Carrier_siamese_eater_tail	", /* 11	5	1 in 48405800	24.5	*/ "xs11_354c826"),
(/* 197	Transboatoncap.png	*/ "Trans-boat_on_cap	", /* 13	4	1 in 48778200	24.5	*/ "xs13_2560uic"),
(/* 198	Xs14_g88m552z121.png	*/ "Ortho-bun_on_loaf	", /* 14	5	1 in 52557900	24.6	*/ "xs14_g88m552z121"),
(/* 199	Xs16_0ggs2qrz32.png	*/ "Block_on_cover_siamese_eater	", /* 16	5	1 in 53092200	24.7	*/ "xs16_0ggs2qrz32"),
(/* 200	Xs15_25960uic.png	*/ "Para-loaf_and_cap	", /* 15	4	1 in 54457600	24.7	*/ "xs15_25960uic"),
(/* 201	Longhorn.png	*/ "Longhorn	", /* 19	4	1 in 55524900	24.7	*/ "xs19_69b88gz69d11"),
(/* 202	Long3snake.png	*/ "Long^3_snake	", /* 9	5	1 in 55684300	24.7	*/ "xs9_31248go"),
(/* 203	Hatwithcape.png	*/ "Hat_with_cape	", /* 13	5	1 in 55685400	24.7	*/ "xs13_17871ac"),
(/* 204	Xs15_25a8ob96.png	*/ "unnamed	", /* 15	5	1 in 56786200	24.7	*/ "xs15_25a8ob96"),
(/* 205	Parabookendandhouse.png	*/ "Para-bookend_and_house	", /* 16	5	1 in 56802800	24.7	*/ "xs16_m2s079cz11"),
    ];

    #[test]
    #[ignore = "broken"]
    fn all_still_lifes() {
        for (name, apgcode) in STILL_LIFES {
            // if name != &"Dead_spark_coil	" {
            //     continue;
            // }

            let mut pattern = Pattern::from_apgcode(apgcode, LifeOptions::default());
            assert_eq!(pattern.to_apgcode(), *apgcode, "Apgcode parse: {}", name);

            let patterns = identify(&mut pattern.life);

            assert_eq!(
                patterns[0].to_apgcode(),
                *apgcode,
                "Pattern identify: {} found \n{}",
                name,
                patterns[0]
            );
        }
    }

    #[rustfmt::skip]
    const OSCILLATORS: &[(&str, &str)] = &[
// https://conwaylife.com/wiki/List_of_common_oscillators
// Pattern	apgcode	Period	Heat	Minimum # of cells	Glider cost	Relative frequency	Rotor	Bushing type
(/* 1*/	"Blinker",	"xp2_7" /* 2	4	3	2	0.9898377	Pole 2	Dot */ ),
(/* 2*/	"Toad", "xp2_7e" /*	2	8	6	3	1 in 132.41	Toad	Toad */ ),
(/* 3*/	"Beacon", "xp2_318c" /*	2	2	6	3	1 in 423.43	Diagonal on-off	Beacon */ ),
(/* 4*/	"Pulsar", "xp3_co9nas0san9oczgoldlo0oldlogz1047210127401" /*	3	42.7	48	3	1 in 4109.8	Pulsar	Pulsar */ ),
(/* 5*/	"Pentadecathlon", "xp15_4r4z4r4" /*	15	22.4	12	3	1 in 381776	Pentadecathlon	- */ ),
(/* 6*/	"Clock", "xp2_2a54" /*	2	8	6	4	1 in 654985	Clock (rotor)	Clock */ ),
(/* 7*/	"Bipole", "xp2_31ago" /*	2	4	8	4	1 in 3.130 million	Pole 2	Bipole */ ),
(/* 8*/	"Quadpole", "xp2_0g0k053z32" /*	2	8	10	5	1 in 3.500 million	Pole 4	Quadpole */ ),
(/* 9*/	"Great_on-off", "xp2_g8gid1e8z1226" /*	2	2	18	5	1 in 5.081 million	Diagonal on-off	Cavity */ ),
(/* 10*/	"Figure_eight", "xp8_gk2gb3z11" /*	8	16.5	12	4	1 in 10.66 million	Figure eight	- */ ),
(/* 11*/	"Spark_coil", "xp2_rhewehr" /*	2	2	18	4	1 in 15.53 million	Orthogonal on-off	Spark coil */ ),
(/* 12*/	"Mold", "xp4_37bkic" /*	4	7	12	5	1 in 18.64 million	Mold	Loaf */ ),
(/* 13*/	"Quadpole_tie_ship", "xp2_31a08zy0123cko" /*	2	8	16	5	1 in 45.35 million	Pole 4	Quadpole */ ),
(/* 14*/	"Tripole", "xp2_g0k053z11" /*	2	6	9	5	1 in 92.25 million	Pole 3	Tripole */ ),
(/* 15*/	"Mazing", "xp4_ssj3744zw3" /*	4	14	12	5	1 in 97.85 million	Mazing	- */ ),
(/* 16*/	"Blocker", "xp8_g3jgz1ut" /*	8	9	15	5	1 in 140 million	Blocker	Blocker */ ),
(/* 17*/	"Jam", "xp3_695qc8zx33" /*	3	14.3	13	5	1 in 186 million	Jam	Loaf */ ),
(/* 18*/	"Trans-queen_bee_shuttle", "xp30_w33z8kqrqk8zzzx33" /*	30	17.9	20	5	1 in 324 million	Queen bee	Blocks */ ),
(/* 19*/	"Cis-queen_bee_shuttle", "xp30_w33z8kqrqk8zzzw33" /*	30	17.9	20	5	1 in 330 million	Queen bee	Blocks */ ),
(/* 20*/	"Trans-beacon_on_table", "xp2_wbq23z32" /*	2	2	12	5	1 in 390 million	Diagonal on-off	Beacon */ ),
(/* 21*/	"Cis-beacon_on_table", "xp2_318c0f9" /*	2	2	12	5	1 in 398 million	Diagonal on-off	Beacon */ ),
(/* 22*/	"Cis-beacon_on_dock", "xp2_j1u062goz11" /*	2	2	16	4	1 in 418 million	Diagonal on-off	Beacon */ ),
(/* 23*/	"Test_tube_baby", "xp2_31egge13" /*	2	2	14	5	1 in 499 million	Orthogonal on-off	Test tube baby */ ),
(/* 24*/	"Achim's_p4", "xp4_8eh5e0e5he8z178a707a871" /*	4	4	40	5	1 in 518 million	1-2-3-4	? */ ),
(/* 25*/	"Cis-beacon_on_anvil", "xp2_0c813z255d1e8" /*	2	2	19	5	1 in 536 million	Diagonal on-off	Beacon */ ),
(/* 26*/	"Octagon_2", "xp5_idiidiz01w1" /*	5	16	16	5	1 in 621 million	Octagon 2	n/a */ ),
(/* 27*/	"Unix", "xp6_ccb7w66z066" /*	6	11.3	16	5	1 in 630 million	Unix	Unix */ ),
(/* 28*/	"1_beacon", "xp2_ca9baiczw32" /*	2	2	18	5	1 in 642 million	Diagonal on-off	1 beacon */ ),
(/* 29*/	"Tub_test_tube_baby", "xp2_25a88gz8ka221" /*	2	2	16	6	1 in 688 million	Orthogonal on-off	Test tube baby */ ),
(/* 30*/	"Trans-beacon_on_dock", "xp2_03lk453z6401" /*	2	2	16	5	1 in 907 million	Diagonal on-off	Beacon */ ),
(/* 31*/	"Tumbler", "xp14_j9d0d9j" /*	14	10.3	16	5	1 in 944 million	Tumbler	- */ ),
(/* 32*/	"Block_on_griddle", "xp2_c8b8aczw33" /*	2	6	15	5	1 in 1.01 billion	Flutter	Griddle */ ),
(/* 33*/	"Fore_and_back", "xp2_ca1n0brz330321" /*	2	4	24	5	1 in 1.48 billion	Pole 2	Snake pit */ ),
(/* 34*/	"Cis-beacon_up_on_long_bookend", "xp2_3hu0og26" /*	2	2	14	5	1 in 1.56 billion	Diagonal on-off	Beacon */ ),
(/* 35*/	"Cis-beacon_on_cap", "xp2_318c0f96" /*	2	2	14	5	1 in 1.67 billion	Diagonal on-off	Beacon */ ),
(/* 36*/	"Cis-beacon_down_on_long_bookend", "xp2_318c0fho" /*	2	2	14	5	1 in 1.87 billion	Diagonal on-off	Beacon */ ),
(/* 37*/	"Trans-block_on_candlefrobra", "xp3_025qzrq221" /*	3	6	16	5	1 in 2.06 billion	Candlefrobra	Candlefrobra */ ),
(/* 38*/	"Mango_with_beacon_on_dock", "xp2_wgj1u0og26z25421" /*	2	2	21	5	1 in 2.30 billion	Diagonal on-off	Beacon */ ),
(/* 39*/	"Trans-beacon_down_on_long_bookend", "xp2_w8o0uh3z32" /*	2	2	14	5	1 in 3.77 billion	Diagonal on-off	Beacon */ ),
(/* 40*/	"Caterer", "xp3_4hh186z07", /*  "3"  14.7	12	5	1 in 3.97 billion	Caterer	n/a */ ),
(/* 41*/	"Coe's_p8", "xp8_wgovnz234z33" /*	8	12.5	17	5	1 in 4.27 billion	Coe's p8	Coe's p8 */ ),
(/* 42*/	"Trans-beacon_on_cap", "xp2_318czw3553" /*	2	2	14	5	1 in 4.95 billion	Diagonal on-off	Beacon */ ),
(/* 43*/	"xp2_8e1t2gozw23", "xp2_8e1t2gozw23" /*	2	2	16	5	1 in 5.64 billion	Diagonal on-off	1 Beacon */ ),
(/* 44*/	"Beehive_on_griddle", "xp2_4k1v0ciczw11" /*	2	6	17	6	1 in 5.67 billion	Flutter	Griddle */ ),
(/* 45*/	"Boat_tie_spark_coil", "xp2_xrhewehrz253" /*	2	2	23	5	1 in 5.90 billion	Orthogonal on-off	Spark coil */ ),
(/* 46*/	"Cis-block_on_candlefrobra", "xp3_025qz32qq1" /*	3	6	16	5	1 in 5.94 billion	Candlefrobra	Candlefrobra */ ),
(/* 47*/	"xp2_69b8baiczx32", "xp2_69b8baiczx32" /*	2	2	20	5	1 in 6.00 billion	Diagonal on-off	1 beacon */ ),
(/* 48*/	"Trans-beacon_up_on_long_bookend", "xp2_03lk46z6401" /*	2	2	14	5	1 in 6.29 billion	Diagonal on-off	Beacon */ ),
(/* 49*/	"Bipole_tie_ship", "xp2_xoga13z653" /*	2	4	14	5	1 in 7.26 billion	Pole 2	Bipole */ ),
(/* 50*/	"Boat_tie_bipole", "xp2_xoga13z253" /*	2	4	13	5	1 in 7.85 billion	Pole 2	Bipole */ ),
(/* 51*/	"Boat_on_griddle", "xp2_c8b8acz0253" /*	2	6	16	6	1 in 10.9 billion	Flutter	Griddle */ ),
(/* 52*/	"Four_boats", "xp2_8ki1688gzx3421" /*	2	4	16	6	1 in 11.3 billion	~	~ */ ),
(/* 53*/	"Boat_tie_quadpole", "xp2_256o8gzy120ago" /*	2	8	15	6	1 in 11.6 billion	Pole 4	Quadpole */ ),
(/* 54*/	"Monotub_spark_coil", "xp2_gbhewehrz01" /*	2	2	19	6	1 in 15.1 billion	Orthogonal on-off	Spark coil */ ),
(/* 55*/	"Unnamed", "xp2_og2t1e8z6221" /*	2	2	18	5	1 in 15.3 billion	Diagonal on-off	1 beacon */ ),
(/* 56*/	"xp2_caabaiczw32", "xp2_caabaiczw32" /*	2	2	18	5	1 in 16.3 billion	Diagonal on-off	1 beacon */ ),
(/* 57*/	"Eater_plug", "xp2_wo44871z6221" /*	2	2	14	6	1 in 17.1 billion	Diagonal on-off	Cavity */ ),
(/* 58*/	"Odd_test_tube_baby", "xp2_03544oz4a511" /*	2	2	15	6	1 in 17.3 billion	Orthogonal on-off	Test tube baby */ ),
(/* 59*/	"Trans-beacon_on_bookend_siamese_table", "xp2_09v0c813z321" /*	2	2	17	6	1 in 17.7 billion	Diagonal on-off	Beacon */ ),
(/* 60*/	"G_and_spark_coil", "xp2_mlhewehrz1" /*	2	2	21	6	1 in 19.6 billion	Orthogonal on-off	Spark coil */ ),
(/* 61*/	"xp2_69b8b9iczx32", "xp2_69b8b9iczx32" /*	2	2	20	6	1 in 21.5 billion	Diagonal on-off	1 beacon */ ),
(/* 62*/	"Cis-beehive_on_candlefrobra", "xp3_4a422hu0696" /*	3	6	18	7	1 in 24.9 billion	Candlefrobra	Candlefrobra */ ),
(/* 63*/	"xp2_33gv1og26", "xp2_33gv1og26" /*	2	2	17	5	1 in 27.8 billion	Diagonal on-off	Beacon */ ),
(/* 64*/	"Beacon_on_cover", "xp2_wbq2sgz32" /*	2	2	14	6	1 in 28.4 billion	Diagonal on-off	Beacon */ ),
(/* 65*/	"Blocks_on_griddle", "xp2_rb88brz0103" /*	2	6	19	5	1 in 36.1 billion	Flutter	Griddle */ ),
(/* 66*/	"Open_cuphook", "xp3_6a889b8ozx33" /*	3	2	18	6	1 in 37.3 billion	Cuphook	Cuphook */ ),
(/* 67*/	"Cis-boat_down_on_candlefrobra", "xp3_4a422hu06a4" /*	3	6	17	5	1 in 41.5 billion	Candlefrobra	Candlefrobra */ ),
(/* 68*/	"xp2_6a88baiczx32", "xp2_6a88baiczx32" /*	2	2	18	5	1 in 41.8 billion	Diagonal on-off	1 beacon */ ),
(/* 69*/	"Beacon_on_inflected_clip", "xp2_0318cz259d1d96" /*	2	2	22	6	1 in 42.0 billion	Diagonal on-off	Beacon */ ),
(/* 70*/	"xp2_gk2t2sgzw23", "xp2_gk2t2sgzw23" /*	2	3	16	6	1 in 44 billion	Flutter	Griddle */ ),
(/* 71*/	"Griddle_and_table", "xp2_4k1v0f9zw11" /*	2	6	17	6	1 in 44 billion	Flutter	Griddle */ ),
(/* 72*/	"Racetrack_and_ortho-beacon", "xp2_04aab96zc813" /*	2	2	18	5	1 in 46 billion	Diagonal on-off	Beacon */ ),
(/* 73*/	"Fox", "xp2_g45h1sz03",  /* 2 	18	12	6	1 in 47 billion	~	~ */ ),
(/* 74*/	"Cis-bipole_on_dock", "xp2_31agozca22ac" /*	2	2	18	5	1 in 47 billion	Pole 2	Bipole */ ),
(/* 75*/	"Fumarole", "xp5_3pmwmp3zx11" /*	5	12.8	18	6	1 in 56 billion	~	~ */ ),
(/* 76*/	"Beacon_on_racetrack", "xp2_0318cz69d552" /*	2	2	18	7	1 in 63 billion	Diagonal on-off	Beacon */ ),
(/* 77*/	"Beehive_test_tube_baby", "xp2_03544ozcid11" /*	2	2	17	6	1 in 69 billion	Orthogonal on-off	Test tube baby */ ),
(/* 78*/	"Cis-beacon_down_on_trans-mango-with-long-leg", "xp2_gwci96zh15dz11" /*	2	2	19	7	1 in 69 billion	Diagonal on-off	Beacon */ ),
(/* 79*/	"Unnamed", "xp2_og2t1acz6221" /*	2	2	18	8	1 in 62 billion	Diagonal on-off	1 beacon */ ),
(/* 80*/	"Cis-boat_up_on_candlefrobra", "xp3_2530fh884a4" /*	3	6	17	5	1 in 73 billion	Candlefrobra	Candlefrobra */ ),
(/* 81*/	"Loaf_on_griddle", "xp2_04a96z31d153" /*	2	3	18	5	1 in 76 billion	Flutter	Griddle */ ),
(/* 82*/	"Lightbulb_and_cis-hook", "xp2_03lkmzojc0cjo" /*	2	3	24	6	1 in 79 billion	V triplet	Lightbulb */ ),
(/* 83*/	"Trans-boat_test_tube_baby", "xp2_03544ozca511" /*	2	2	16	8	1 in 82 billion	Orthogonal on-off	Test tube baby */ ),
(/* 84*/	"Pond_on_griddle", "xp2_06996z31d153" /*	2	3	19	6	1 in 84 billion	Flutter	Griddle */ ),
(/* 85*/	"Cis-beacon_up_on_tub-with-long-leg", "xp2_c813z3115a4" /*	2	2	15	6	1 in 88 billion	Diagonal on-off	Beacon */ ),
(/* 86*/	"Cis-beacon_up_on_beehive-with-long-leg", "xp2_c813z311dic" /*	2	2	17	8	1 in 97 billion	Diagonal on-off	Beacon */ ),
    ];

    #[test]
    #[ignore = "broken"]
    fn all_oscillators() {
        for (name, apgcode) in OSCILLATORS {
            // if name != &"Pulsar" {
            //     continue;
            // }

            let mut pattern = Pattern::from_apgcode(apgcode, LifeOptions::default());
            assert_eq!(pattern.to_apgcode(), *apgcode, "Apgcode parse: {}", name);

            let patterns = identify(&mut pattern.life);

            assert_eq!(
                patterns[0].to_apgcode(),
                *apgcode,
                "Pattern identify: {} found \n{}",
                name,
                patterns[0]
            );
        }
    }

    // SPACESHIPS:
    // - Option 1: Detect when Pattern groups collide with edge and delete them/check for spaceships
    // - Option 2: Somehow expanding grid and check when they get far
    // - Option

    #[rustfmt::skip]
    const SPACESHIPS: &[(&str, &str)] = &[
// Image	Pattern	Period	Heat	Speed	Minimum # of cells	Glider cost	Relative frequency	Frequency class	Apgcode
(/*1	Glider.png	*/ "Glider", /*4	4	c/4 diagonal	5	2	1 in 1.0019127 (99.809%)	1.8*/ "xq4_153"),
(/*2	Lwss.png	*/ "Lightweight_spaceship", /*4	11	2c/4 orthogonal	9	3	1 in 685.81 (0.146%)	11.2*/ "xq4_6frc"),
(/*3	Mwss.png	*/ "Middleweight_spaceship", /*4	15	2c/4 orthogonal	11	3	1 in 2,603.1 (0.038%)	13.2*/ "xq4_27dee6"),
(/*4	Hwss.png	*/ "Heavyweight_spaceship", /*4	19	2c/4 orthogonal	13	3	1 in 14,979 (0.007%)	15.7*/ "xq4_27deee6"),
    ];

    #[test]
    #[ignore = "broken"]
    fn all_spaceships() {
        for (name, apgcode) in SPACESHIPS {
            // if name != &"Pulsar" {
            //     continue;
            // }

            let mut pattern = Pattern::from_apgcode(apgcode, LifeOptions::default());
            assert_eq!(pattern.to_apgcode(), *apgcode, "Apgcode parse: {}", name);

            let patterns = identify(&mut pattern.life);

            assert_eq!(
                patterns[0].to_apgcode(),
                *apgcode,
                "Pattern identify: {} found \n{}",
                name,
                patterns[0]
            );
        }
    }

    #[test]
    #[ignore = "Requires a rethink"]
    fn messless_diehard() {
        const DIEHARD: &str = "\
!Name: Die hard
!A methuselah that vanishes at generation 130, which is conjectured to be maximal for patterns of 7 or fewer cells.
!https://www.conwaylife.com/wiki/index.php?title=Die_hard
......O
OO
.O...OOO";

        let mut life = Life::new_ex(pos(64, 64), LifeOptions {
            algo: crate::life::LifeAlgoSelect::Cached,
            ..Default::default()
        });
        let die_hard_life = Life::from_plaintext(DIEHARD, LifeOptions::default());
        life.paste(&die_hard_life, pos(32, 32), None);

        let _patterns = identify(&mut life);

        // assert_eq!(patterns[0].metadata, PatternMetadata {
        //     classification: Some(Classification::Messless),
        //     ..Default::default()
        // });
    }
}
