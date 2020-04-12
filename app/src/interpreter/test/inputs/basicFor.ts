import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
    var N := 0
in
    (for r := 0 to 10 do
        N := N + r
    ); N
end
*/

export const basicForInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_41a70f42-7e5f-4521-9cd4-8557823b9f86' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'N' }, { CONST: 0 }] },
                { MOVE: [{ LOCAL: 'r' }, { CONST: 0 }] },
                {
                    CJUMP: [
                        'LE',
                        { LOCAL: 'r' },
                        { CONST: 10 },
                        '-start_d1580694-119f-4586-bfab-94b791b9a1f7',
                        '-for-done_34570100-ed3f-4e93-88fb-601693d15a5d',
                    ],
                },
                { LABEL: '-for-done_34570100-ed3f-4e93-88fb-601693d15a5d' },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'N' }] },
                {
                    JUMP: [
                        { NAME: '-done_75f884ab-2ae2-4917-b0bf-7a08d50a3a12' },
                        ['-done_75f884ab-2ae2-4917-b0bf-7a08d50a3a12'],
                    ],
                },
                { LABEL: '-newblock-cjump_af2894fa-2d2c-4546-aa29-531351e087f5' },
                { LABEL: '-start_d1580694-119f-4586-bfab-94b791b9a1f7' },
                { MOVE: [{ LOCAL: 'N' }, { BINOP: ['PLUS', { LOCAL: 'N' }, { LOCAL: 'r' }] }] },
                {
                    CJUMP: [
                        'GE',
                        { LOCAL: 'r' },
                        { CONST: 10 },
                        '-for-done_34570100-ed3f-4e93-88fb-601693d15a5d',
                        '-continue_9bd7777a-96b8-40f5-96cb-fb4c1a6028f6',
                    ],
                },
                { LABEL: '-continue_9bd7777a-96b8-40f5-96cb-fb4c1a6028f6' },
                { MOVE: [{ LOCAL: 'r' }, { BINOP: ['PLUS', { LOCAL: 'r' }, { CONST: 1 }] }] },
                {
                    JUMP: [
                        { NAME: '-start_d1580694-119f-4586-bfab-94b791b9a1f7' },
                        ['-start_d1580694-119f-4586-bfab-94b791b9a1f7'],
                    ],
                },
                { LABEL: '-newblock-cjump_2c36d004-26e8-4e31-b21e-5166bf8a32eb' },
                {
                    JUMP: [
                        { NAME: '-continue_9bd7777a-96b8-40f5-96cb-fb4c1a6028f6' },
                        ['-continue_9bd7777a-96b8-40f5-96cb-fb4c1a6028f6'],
                    ],
                },
                { LABEL: '-newblock-jump_e93f178d-3e8d-43be-8fab-98ec8d90341a' },
                {
                    JUMP: [
                        { NAME: '-for-done_34570100-ed3f-4e93-88fb-601693d15a5d' },
                        ['-for-done_34570100-ed3f-4e93-88fb-601693d15a5d'],
                    ],
                },
                { LABEL: '-done_75f884ab-2ae2-4917-b0bf-7a08d50a3a12' },
            ],
            frame: {
                label: '_tigermain_0ae2e6e2-0242-4ba7-ac72-c88a1cb0d792',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
