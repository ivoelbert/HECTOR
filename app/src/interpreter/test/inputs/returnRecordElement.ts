import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
    type rectype = {name : string, age : int}
    var rec1 : rectype := rectype {name="Nobody", age=42}
in
    rec1.age
end
*/

export const returnRecordElementTestInput: Frag[] = [
    { ConstString: ['Nobody_214f32b3-eda7-48d2-a13d-45b086179596', 'Nobody'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_cf83351d-ceea-4339-89b0-70ed5284bbc9' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'rec1' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 2 },
                                    { NAME: 'Nobody_214f32b3-eda7-48d2-a13d-45b086179596' },
                                    { CONST: 42 },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'rec1' }, { CONST: 1 }] } },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_d17c0819-4b5c-40c2-87e1-401323e0d9d2' },
                        ['-done_d17c0819-4b5c-40c2-87e1-401323e0d9d2'],
                    ],
                },
                { LABEL: '-done_d17c0819-4b5c-40c2-87e1-401323e0d9d2' },
            ],
            frame: {
                label: '_tigermain_188860f5-806f-42c9-a21d-1f68dd413cd1',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
