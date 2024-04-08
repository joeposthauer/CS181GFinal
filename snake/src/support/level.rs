pub struct Level {
    name: String,
    bg: SheetRegion,
    grid: Grid<u8>,
    tileset: Tileset,
    starts: Vec<(EntityType, Vec2)>,
}

impl Level {
    pub fn from_str(s: &str) -> Self {
        enum State {
            Metadata,
            Legend,
            Map,
            Starts,
            Done,
        }
        impl State {
            fn next(self) -> Self {
                match self {
                    Self::Metadata => Self::Legend,
                    Self::Legend => Self::Map,
                    Self::Map => Self::Starts,
                    Self::Starts => Self::Done,
                    Self::Done => Self::Done,
                }
            }
        }
        let mut state = State::Metadata;
        let mut name = None;
        let mut dims = None;
        let mut legend: HashMap<String, (u8, TileData)> = std::collections::HashMap::new();
        let mut grid = vec![];
        let mut starts = vec![];
        let mut bg = SheetRegion::ZERO;
        for line in s.lines() {
            if line.is_empty() {
                continue;
            } else if line.chars().all(|c| c == '=') {
                state = state.next();
            } else {
                match state {
                    State::Metadata => {
                        let mut chunks = line.split_whitespace();
                        let md = chunks
                            .next()
                            .expect("No metadata decl string in metadata line {line}");
                        if md == "bg" {
                            if bg.w != 0 {
                                panic!("Two bg entries in metadata");
                            }
                            bg = SheetRegion::rect(
                                u16::from_str(chunks.next().expect("No x in metadata line {line}"))
                                    .expect("Couldn't parse x as u16 in {line}"),
                                u16::from_str(chunks.next().expect("No y in metadata line {line}"))
                                    .expect("Couldn't parse y as u16 in {line}"),
                                i16::from_str(
                                    chunks.next().expect("No width in metadata line {line}"),
                                )
                                .expect("Couldn't parse width as i16 in {line}"),
                                i16::from_str(
                                    chunks.next().expect("No height in metadata line {line}"),
                                )
                                .expect("Couldn't parse height as i16 in {line}"),
                            )
                            .with_depth(17);
                        } else {
                            if name.is_some() {
                                panic!("Two name entries in metadata");
                            }
                            name = Some(md.to_string());
                            dims = Some((
                                u16::from_str(
                                    chunks.next().expect("No width in metadata line {line}"),
                                )
                                .expect("Couldn't parse width as u16 in {line}"),
                                u16::from_str(
                                    chunks.next().expect("No height in metadata line {line}"),
                                )
                                .expect("Couldn't parse height as u16 in {line}"),
                            ));
                        }
                    }
                    State::Legend => {
                        let mut chunks = line.split_whitespace();
                        let sym = chunks.next().expect("Couldn't get tile symbol in {line}");
                        assert!(!legend.contains_key(sym), "Symbol {sym} already in legend");
                        let flags = chunks
                            .next()
                            .expect("Couldn't get tile flags in {line}")
                            .to_lowercase();
                        assert!(flags == "o" || flags == "s", "The only valid flags are o(pen) or s(olid), but you could parse other kinds here in {line}");
                        let x =
                            u16::from_str(chunks.next().expect("No sheet x in legend line {line}"))
                                .expect("Couldn't parse sheet x as u16 in {line}");
                        let y =
                            u16::from_str(chunks.next().expect("No sheet y in legend line {line}"))
                                .expect("Couldn't parse sheet y as u16 in {line}");
                        let w =
                            i16::from_str(chunks.next().expect("No sheet w in legend line {line}"))
                                .expect("Couldn't parse sheet w as i16 in {line}");
                        let h =
                            i16::from_str(chunks.next().expect("No sheet h in legend line {line}"))
                                .expect("Couldn't parse sheet h as i16 in {line}");
                        let data = TileData {
                            solid: flags == "s",
                            sheet_region: SheetRegion::new(0, x, y, 16, w, h),
                        };
                        legend.insert(sym.to_string(), (legend.len() as u8, data));
                    }
                    State::Map => {
                        let old_len = grid.len();
                        grid.extend(line.split_whitespace().map(|sym| legend[sym].0));
                        assert_eq!(
                            old_len + dims.unwrap().0 as usize,
                            grid.len(),
                            "map line is too short: {line} for map dims {dims:?}"
                        );
                    }
                    State::Starts => {
                        let mut chunks = line.split_whitespace();
                        let etype = chunks
                            .next()
                            .expect("Couldn't get entity start type {line}");
                        let etype = match etype {
                            "snake" => EntityType::Snake,
                            "apple" => EntityType::Food,
                            _ => panic!("Unrecognized entity type in {line}"),
                        };
                        let x =
                            u16::from_str(chunks.next().expect("No x coord in start line {line}"))
                                .expect("Couldn't parse x coord as u16 in {line}");
                        let y =
                            u16::from_str(chunks.next().expect("No y coord in start line {line}"))
                                .expect("Couldn't parse y coord as u16 in {line}");
                        starts.push((
                            etype,
                            Vec2 {
                                x: (x as usize * TILE_SZ) as f32 + TILE_SZ as f32 / 2.0,
                                y: ((dims.unwrap().1 - y) as usize * TILE_SZ) as f32
                                    - TILE_SZ as f32 / 2.0,
                            },
                        ));
                    }
                    State::Done => {
                        panic!("Unexpected file content after parsing finished in {line}")
                    }
                }
            }
        }
        assert_ne!(name, None);
        let name = name.unwrap();
        assert_ne!(dims, None);
        let (w, h) = dims.unwrap();
        assert!(!legend.is_empty());
        assert_eq!(grid.len(), w as usize * h as usize);
        let mut tiles: Vec<(u8, TileData)> = legend.into_values().collect();
        tiles.sort_by_key(|(num, _)| *num);
        Self {
            bg,
            name: name.to_string(),
            grid: Grid::new(w as usize, h as usize, grid),
            tileset: Tileset {
                tiles: tiles.into_iter().map(|(_num, val)| val).collect(),
            },
            starts,
        }
    }
}