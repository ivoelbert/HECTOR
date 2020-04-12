import { Frag } from '../../treeTypes';

/*
PROGRAM:

42
*/

export const returnNumberTestInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_985a300d-9426-4b83-93cb-f2e17411265c' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ GLOBAL: 'rv' }, { CONST: 42 }] },
                {
                    JUMP: [
                        { NAME: '-done_f4c68b19-2096-4725-a6d9-5832a706c3e6' },
                        ['-done_f4c68b19-2096-4725-a6d9-5832a706c3e6'],
                    ],
                },
                { LABEL: '-done_f4c68b19-2096-4725-a6d9-5832a706c3e6' },
            ],
            frame: {
                label: '_tigermain_e7b4e0fb-5bef-4bd6-b10a-85fd1d3537f5',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
