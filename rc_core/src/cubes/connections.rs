pub const DEFAULT_CONNECTION: CubeConnections = CubeConnections {
    id: 0, // (default)
    connections: &[
        CubeConnection {
            direction: (0, 0, 1),
            position: (0, 0, 0),
        },
        CubeConnection {
            direction: (0, 1, 0),
            position: (0, 0, 0),
        },
        CubeConnection {
            direction: (1, 0, 0),
            position: (0, 0, 0),
        },
        CubeConnection {
            direction: (0, 0, -1),
            position: (0, 0, 0),
        },
        CubeConnection {
            direction: (0, -1, 0),
            position: (0, 0, 0),
        },
        CubeConnection {
            direction: (-1, 0, 0),
            position: (0, 0, 0),
        },
    ],
};

pub const CUBE_CONNECTIONS: &[CubeConnections] = &[
    CubeConnections {
        id: 606866102, // Protonium Clasp
        connections: &[
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 38, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 48, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 58, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 68, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 78, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 88, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 98, 25),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 108, 25),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 38, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 48, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 58, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 68, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 78, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 88, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 98, -24),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 108, -24),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 38, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 48, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 58, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 68, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 78, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 88, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 98, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (25, 108, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 38, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 48, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 58, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 68, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 78, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 88, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 98, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-24, 108, 0),
            },
            /*CubeConnection { // assumed by game client (but not useful)
                direction: (0, -1, 0),
                position: (0, 0, 0),
            },*/
        ],
    },
    CubeConnections {
        id: 3950293873, // Protonium Crystal
        connections: &[
            // -y (bottom)
            CubeConnection { // assumed by game client
                direction: (0, -1, 0),
                position: (0, 0, 0),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (1, 0, 0),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (0, 0, 1),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (1, 0, 1),
            },
            // +y (top)
            CubeConnection {
                direction: (0, 1, 0),
                position: (0, 9, 0),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (1, 9, 0),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (0, 9, 1),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (1, 9, 1),
            },
            // -x (right?)
            CubeConnection {
                direction: (1, 0, 0),
                position: (5, 4, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (5, 4, 1),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (5, 5, 0),
            },
            CubeConnection {
                direction: (1, 0, 0),
                position: (5, 5, 1),
            },
            // -x (left?)
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-4, 4, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-4, 4, 1),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-4, 5, 0),
            },
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-4, 5, 1),
            },
            // +z (back)
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 4, 5),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (1, 4, 5),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (0, 5, 5),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (1, 5, 5),
            },
            // -z (front)
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 4, -4),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (1, 4, -4),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (0, 5, -4),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (1, 5, -4),
            },
            // diagonal faces
            CubeConnection {
                direction: (-1, 0, 0),
                position: (-2, 2, -2),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (-2, 2, -2),
            },

            CubeConnection {
                direction: (1, 0, 0),
                position: (3, 2, -2),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (3, 2, -2),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (3, 2, -2),
            },

            CubeConnection {
                direction: (-1, 0, 0),
                position: (-2, 2, 3),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (-2, 2, 3),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (-2, 2, 3),
            },

            CubeConnection {
                direction: (1, 0, 0),
                position: (3, 2, 3),
            },
            CubeConnection {
                direction: (0, -1, 0),
                position: (3, 2, 3),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (3, 2, 3),
            },

            CubeConnection {
                direction: (-1, 0, 0),
                position: (-2, 6, -2),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (-2, 6, -2),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (-2, 6, -2),
            },

            CubeConnection {
                direction: (1, 0, 0),
                position: (3, 6, -2),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (3, 6, -2),
            },
            CubeConnection {
                direction: (0, 0, -1),
                position: (3, 6, -2),
            },

            CubeConnection {
                direction: (-1, 0, 0),
                position: (-2, 6, 3),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (-2, 6, 3),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (-2, 6, 3),
            },

            CubeConnection {
                direction: (1, 0, 0),
                position: (3, 6, 3),
            },
            CubeConnection {
                direction: (0, 1, 0),
                position: (3, 6, 3),
            },
            CubeConnection {
                direction: (0, 0, 1),
                position: (3, 6, 3),
            },
        ],
    },
    DEFAULT_CONNECTION,
];

#[derive(Copy, Clone)]
pub struct CubeConnections {
    pub id: u32,
    pub connections: &'static [CubeConnection],
}

#[derive(Copy, Clone)]
pub struct CubeConnection {
    pub direction: (i8, i8, i8),
    pub position: (i8, i8, i8),
}
