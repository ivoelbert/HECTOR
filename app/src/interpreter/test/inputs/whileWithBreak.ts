import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
	var n : int := 0
in
    (while 1 do
        (if n >= 10
            then break
            else (n := n + 1)
        )
    );
    n
end
*/

export const whileWithBreakInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_8301a7df-e03e-45bf-86c3-e3c64a92e68f' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'n' }, { CONST: 0 }] },
                { LABEL: '-test_5e43604e-6cba-4a1b-be73-c05b38b07dac' },
                {
                    CJUMP: [
                        'GE',
                        { CONST: 1 },
                        { CONST: 1 },
                        '-body_7727526e-dc96-464a-953c-3bb0cb363f49',
                        '-while-done_4277cf51-a874-4d72-b2a1-e915c393277b',
                    ],
                },
                { LABEL: '-while-done_4277cf51-a874-4d72-b2a1-e915c393277b' },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'n' }] },
                {
                    JUMP: [
                        { NAME: '-done_26c620a8-b8bc-4f79-baab-1f3955ec781e' },
                        ['-done_26c620a8-b8bc-4f79-baab-1f3955ec781e'],
                    ],
                },
                { LABEL: '-newblock-cjump_dd91a999-807e-4a1d-8dd0-89cce36b0602' },
                { LABEL: '-body_7727526e-dc96-464a-953c-3bb0cb363f49' },
                {
                    CJUMP: [
                        'GE',
                        { BINOP: ['UGE', { LOCAL: 'n' }, { CONST: 10 }] },
                        { CONST: 1 },
                        '-then_703e0685-5f60-4e67-b828-397e25216942',
                        '-else_5dd08bfd-d6c0-4917-980c-6b12b90d6e67',
                    ],
                },
                { LABEL: '-else_5dd08bfd-d6c0-4917-980c-6b12b90d6e67' },
                { MOVE: [{ LOCAL: 'n' }, { BINOP: ['PLUS', { LOCAL: 'n' }, { CONST: 1 }] }] },
                { LABEL: '-join_072dc2dd-a2ac-48a0-bb28-702467680dc9' },
                {
                    JUMP: [
                        { NAME: '-test_5e43604e-6cba-4a1b-be73-c05b38b07dac' },
                        ['-test_5e43604e-6cba-4a1b-be73-c05b38b07dac'],
                    ],
                },
                { LABEL: '-newblock-cjump_4ad649b9-1c9c-4d1e-912a-83dbd3aa3d0d' },
                { LABEL: '-then_703e0685-5f60-4e67-b828-397e25216942' },
                {
                    JUMP: [
                        { NAME: '-while-done_4277cf51-a874-4d72-b2a1-e915c393277b' },
                        ['-while-done_4277cf51-a874-4d72-b2a1-e915c393277b'],
                    ],
                },
                { LABEL: '-newblock-jump_039b88f2-6000-4e38-8264-ae3a1033cf5e' },
                {
                    JUMP: [
                        { NAME: '-join_072dc2dd-a2ac-48a0-bb28-702467680dc9' },
                        ['-join_072dc2dd-a2ac-48a0-bb28-702467680dc9'],
                    ],
                },
                { LABEL: '-newblock-jump_a8d6ae99-01a5-47b8-bcaa-4dc8cfbdca51' },
                {
                    JUMP: [
                        { NAME: '-else_5dd08bfd-d6c0-4917-980c-6b12b90d6e67' },
                        ['-else_5dd08bfd-d6c0-4917-980c-6b12b90d6e67'],
                    ],
                },
                { LABEL: '-newblock-jump_96d157ac-4cb5-4dab-90ae-5f675763bc55' },
                {
                    JUMP: [
                        { NAME: '-while-done_4277cf51-a874-4d72-b2a1-e915c393277b' },
                        ['-while-done_4277cf51-a874-4d72-b2a1-e915c393277b'],
                    ],
                },
                { LABEL: '-done_26c620a8-b8bc-4f79-baab-1f3955ec781e' },
            ],
            frame: {
                label: '_tigermain_2c177bf4-31d1-4e1e-bf08-e26882974e27',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
