import { Frag } from '../../treeTypes';

/*
PROGRAM:

let var  b := 1
    type a = int
in
    let var  c := 2
        function a (a : a) : a = a + b + c
        var b := 3
    in
        a(1)
    end
end
*/

export const escapeInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_94818072-f91c-4bc1-956a-22374a606aae' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        {
                            BINOP: [
                                'PLUS',
                                {
                                    BINOP: [
                                        'PLUS',
                                        { LOCAL: 'a' },
                                        {
                                            MEM: {
                                                BINOP: [
                                                    'PLUS',
                                                    {
                                                        MEM: {
                                                            BINOP: [
                                                                'PLUS',
                                                                { GLOBAL: 'fp' },
                                                                { CONST: 0 },
                                                            ],
                                                        },
                                                    },
                                                    { CONST: 1 },
                                                ],
                                            },
                                        },
                                    ],
                                },
                                {
                                    MEM: {
                                        BINOP: [
                                            'PLUS',
                                            {
                                                MEM: {
                                                    BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }],
                                                },
                                            },
                                            { CONST: 2 },
                                        ],
                                    },
                                },
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_e6a7c3a5-b98f-4f5d-8633-2ac07c453c38' },
                        ['-done_e6a7c3a5-b98f-4f5d-8633-2ac07c453c38'],
                    ],
                },
                { LABEL: '-done_e6a7c3a5-b98f-4f5d-8633-2ac07c453c38' },
            ],
            frame: {
                label: 'a_7a52b545-aac2-4763-a3b2-4b307266537c',
                formals: [
                    ['sl', true],
                    ['a', false],
                ],
                memindex: 0,
            },
        },
    },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_1a934839-1d08-479a-9a1d-f18046a06ce4' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 1 }] } },
                        { CONST: 1 },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 2 }] } },
                        { CONST: 2 },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 3 }] } },
                        { CONST: 3 },
                    ],
                },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        {
                            CALL: [
                                { NAME: 'a_7a52b545-aac2-4763-a3b2-4b307266537c' },
                                [{ GLOBAL: 'fp' }, { CONST: 1 }],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_c626972e-faab-4d23-9775-e43e6e15b6f5' },
                        ['-done_c626972e-faab-4d23-9775-e43e6e15b6f5'],
                    ],
                },
                { LABEL: '-done_c626972e-faab-4d23-9775-e43e6e15b6f5' },
            ],
            frame: {
                label: '_tigermain_6d32126b-4dca-4a27-ab2f-91952e1082ef',
                formals: [['sl', true]],
                memindex: 3,
            },
        },
    },
];
