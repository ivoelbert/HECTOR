import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
    var N := 42
in
    N
end
*/

export const returnVariableTestInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_f2b3fd6d-36c5-47ba-8722-0e502fb350f9' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'N' }, { CONST: 42 }] },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'N' }] },
                {
                    JUMP: [
                        { NAME: '-done_50af5d36-89b6-468e-9681-3ec6aa31313e' },
                        ['-done_50af5d36-89b6-468e-9681-3ec6aa31313e'],
                    ],
                },
                { LABEL: '-done_50af5d36-89b6-468e-9681-3ec6aa31313e' },
            ],
            frame: {
                label: '_tigermain_8d9c1f88-5777-443a-8121-bea13609a166',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
