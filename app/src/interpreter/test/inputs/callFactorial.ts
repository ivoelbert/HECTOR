import { Frag } from '../../treeTypes';

/*
let
    function fact (n : int) : int =
        if  n = 0
            then 1
            else n * fact (n - 1)
in
    fact (5)
end
*/

export const callFactorialInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_491299f5-f80a-4e15-ac5e-a35ef73803da' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { BINOP: ['EQ', { LOCAL: 'n' }, { CONST: 0 }] },
                        { CONST: 1 },
                        '-then_1f008d4a-78f3-40a1-80d5-1500edffb16f',
                        '-else_be973ae8-71a1-493d-8fff-87e2e0ff500e',
                    ],
                },
                { LABEL: '-else_be973ae8-71a1-493d-8fff-87e2e0ff500e' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_ec2b6383-0fc7-4b98-bfdd-3a4b55f6c473' },
                        { LOCAL: 'n' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_bce687d2-c5e3-4ef1-83e5-2ff86e610023' },
                        {
                            CALL: [
                                { NAME: 'fact_e0215f2a-d676-4c4d-b9cc-c066949a0517' },
                                [
                                    { GLOBAL: 'fp' },
                                    { BINOP: ['MINUS', { LOCAL: 'n' }, { CONST: 1 }] },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_d24eadd1-d722-4b89-ab57-1457893c9573' },
                        {
                            BINOP: [
                                'MUL',
                                { LOCAL: '-reorder_ec2b6383-0fc7-4b98-bfdd-3a4b55f6c473' },
                                { LOCAL: '-reorder_call_bce687d2-c5e3-4ef1-83e5-2ff86e610023' },
                            ],
                        },
                    ],
                },
                { LABEL: '-join_6f5194a4-5e12-466d-8b25-49cf3b023e31' },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        { LOCAL: '-ifresult_d24eadd1-d722-4b89-ab57-1457893c9573' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_ed4c4ca5-2324-4738-93b9-036277d514e9' },
                        ['-done_ed4c4ca5-2324-4738-93b9-036277d514e9'],
                    ],
                },
                { LABEL: '-newblock-cjump_a882dac7-c198-4d7f-9b56-22ea25151dbd' },
                { LABEL: '-then_1f008d4a-78f3-40a1-80d5-1500edffb16f' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_d24eadd1-d722-4b89-ab57-1457893c9573' },
                        { CONST: 1 },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_6f5194a4-5e12-466d-8b25-49cf3b023e31' },
                        ['-join_6f5194a4-5e12-466d-8b25-49cf3b023e31'],
                    ],
                },
                { LABEL: '-newblock-jump_bfb1377b-24e6-45b6-85ec-b8c580abd9b5' },
                {
                    JUMP: [
                        { NAME: '-else_be973ae8-71a1-493d-8fff-87e2e0ff500e' },
                        ['-else_be973ae8-71a1-493d-8fff-87e2e0ff500e'],
                    ],
                },
                { LABEL: '-done_ed4c4ca5-2324-4738-93b9-036277d514e9' },
            ],
            frame: {
                label: 'fact_e0215f2a-d676-4c4d-b9cc-c066949a0517',
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
                { LABEL: '-blockfirst_01af7bb5-134e-4479-9271-d11cac600b59' },
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
                                { NAME: 'fact_e0215f2a-d676-4c4d-b9cc-c066949a0517' },
                                [{ GLOBAL: 'fp' }, { CONST: 5 }],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_0ab52c2b-8595-482f-9071-29f1ee5ce4e3' },
                        ['-done_0ab52c2b-8595-482f-9071-29f1ee5ce4e3'],
                    ],
                },
                { LABEL: '-done_0ab52c2b-8595-482f-9071-29f1ee5ce4e3' },
            ],
            frame: {
                label: '_tigermain_53253d6c-0d09-49ed-a718-fa5f61c2b415',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
