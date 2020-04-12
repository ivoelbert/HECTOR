import { Frag } from '../../treeTypes';

/*
PROGRAM:

(print("perro"); 0)
*/

export const printPerroInput: Frag[] = [
    { ConstString: ['perro_c821a070-f086-4eab-9b18-892512c393f0', 'perro'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_6e335f52-68a6-47e0-8120-37fffe828a9a' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'print' },
                            [{ NAME: 'perro_c821a070-f086-4eab-9b18-892512c393f0' }],
                        ],
                    },
                },
                { EXP: { CONST: 0 } },
                { MOVE: [{ GLOBAL: 'rv' }, { CONST: 0 }] },
                {
                    JUMP: [
                        { NAME: '-done_4c4837dd-7501-4ba3-b8ab-91f7466ec4dd' },
                        ['-done_4c4837dd-7501-4ba3-b8ab-91f7466ec4dd'],
                    ],
                },
                { LABEL: '-done_4c4837dd-7501-4ba3-b8ab-91f7466ec4dd' },
            ],
            frame: {
                label: '_tigermain_3e2075c4-6ccb-44d9-afa4-52f1030b1196',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
