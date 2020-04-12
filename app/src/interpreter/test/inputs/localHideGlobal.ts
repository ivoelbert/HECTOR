import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
	var a :=0
	function g (a : int) : int = a
in
	g (2)
end
*/

export const localHideGlobalInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_b54107b2-7faf-4638-87d5-98509874d8c4' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'a' }] },
                {
                    JUMP: [
                        { NAME: '-done_94de2092-48c1-4f77-8401-53ac448a4101' },
                        ['-done_94de2092-48c1-4f77-8401-53ac448a4101'],
                    ],
                },
                { LABEL: '-done_94de2092-48c1-4f77-8401-53ac448a4101' },
            ],
            frame: {
                label: 'g_3f1ca239-5925-4574-a22d-817dd92861a4',
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
                { LABEL: '-blockfirst_c6d22fc3-30f6-4db3-bf01-c319a7235c4d' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'a' }, { CONST: 0 }] },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        {
                            CALL: [
                                { NAME: 'g_3f1ca239-5925-4574-a22d-817dd92861a4' },
                                [{ GLOBAL: 'fp' }, { CONST: 2 }],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_c123fcb9-1d3a-4b7a-a67d-6e6115812feb' },
                        ['-done_c123fcb9-1d3a-4b7a-a67d-6e6115812feb'],
                    ],
                },
                { LABEL: '-done_c123fcb9-1d3a-4b7a-a67d-6e6115812feb' },
            ],
            frame: {
                label: '_tigermain_7995a6ca-f463-4eee-b464-ac4f82016129',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
