import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
    type  arrtype = array of int
    var arr1 : arrtype := arrtype [10] of 0
in
    arr1[5] := 42;
    arr1[5]
end
*/

export const returnArrayElementInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_169a0956-1bcd-41b9-a2f8-9736d3513617' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'arr1' },
                        { CALL: [{ NAME: '+alloc_array' }, [{ CONST: 10 }, { CONST: 0 }]] },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'arr1' }, { CONST: 5 }] } },
                        { CONST: 42 },
                    ],
                },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'arr1' }, { CONST: 5 }] } },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_85c6888b-ddfc-4a18-9dff-80c787591040' },
                        ['-done_85c6888b-ddfc-4a18-9dff-80c787591040'],
                    ],
                },
                { LABEL: '-done_85c6888b-ddfc-4a18-9dff-80c787591040' },
            ],
            frame: {
                label: '_tigermain_a7931528-879d-4aba-82a5-3f99f470ae35',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
