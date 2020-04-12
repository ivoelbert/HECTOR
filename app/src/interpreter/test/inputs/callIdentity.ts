import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
  function id (n : int) : int = n
in
  id (42)
end
*/

export const callIdentityInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_a37e30b7-ca8c-4fa4-a644-301310eba68a' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'n' }] },
                {
                    JUMP: [
                        { NAME: '-done_30139732-2541-4020-9f0e-13967cc47937' },
                        ['-done_30139732-2541-4020-9f0e-13967cc47937'],
                    ],
                },
                { LABEL: '-done_30139732-2541-4020-9f0e-13967cc47937' },
            ],
            frame: {
                label: 'id_3035fec1-5220-4e68-9dcc-b82b03d2fc89',
                formals: [
                    ['sl', true],
                    ['n', false],
                ],
                memindex: 0,
            },
        },
    },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_cea27962-6905-4153-9324-39242527e8c2' },
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
                            CALL: [
                                { NAME: 'id_3035fec1-5220-4e68-9dcc-b82b03d2fc89' },
                                [{ GLOBAL: 'fp' }, { CONST: 42 }],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_59a2d1bb-4377-4a57-83cb-1f0318e114c6' },
                        ['-done_59a2d1bb-4377-4a57-83cb-1f0318e114c6'],
                    ],
                },
                { LABEL: '-done_59a2d1bb-4377-4a57-83cb-1f0318e114c6' },
            ],
            frame: {
                label: '_tigermain_84321dec-1ba4-43dd-a0d7-0b21face5ed8',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
