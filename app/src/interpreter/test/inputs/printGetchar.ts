import { Frag } from '../../treeTypes';

/*
PROGRAM:

(print(getchar()); 0)
*/

export const printGetcharInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_86286b55-2b76-4d77-8c48-21890bdbe6b1' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_0ffd0094-0c08-48aa-bc05-a5605ee82cd9' },
                        { CALL: [{ NAME: 'getchar' }, []] },
                    ],
                },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'print' },
                            [{ LOCAL: '-reorder_call_0ffd0094-0c08-48aa-bc05-a5605ee82cd9' }],
                        ],
                    },
                },
                { MOVE: [{ GLOBAL: 'rv' }, { CONST: 0 }] },
                {
                    JUMP: [
                        { NAME: '-done_ee651f66-15c8-498d-ae6d-709385466d7a' },
                        ['-done_ee651f66-15c8-498d-ae6d-709385466d7a'],
                    ],
                },
                { LABEL: '-done_ee651f66-15c8-498d-ae6d-709385466d7a' },
            ],
            frame: {
                label: '_tigermain_75e35aeb-2156-4404-9ad8-f744b35d5487',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
