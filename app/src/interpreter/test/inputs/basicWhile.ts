import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
	var n : int := 0
in
    (while n < 10 do
        n := n + 1
    );
    n
end
*/

export const basicWhileInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_d94f7a6f-817b-4179-a730-5227c5cfa1ef' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'n' }, { CONST: 0 }] },
                { LABEL: '-test_303a8443-90d2-4fc2-aedd-b7670c32a96d' },
                {
                    CJUMP: [
                        'GE',
                        { BINOP: ['ULT', { LOCAL: 'n' }, { CONST: 10 }] },
                        { CONST: 1 },
                        '-body_5e9319a3-ec22-4366-aaf6-1ee7f06f186e',
                        '-while-done_94761299-2f79-4f88-b947-87ffeedbc449',
                    ],
                },
                { LABEL: '-while-done_94761299-2f79-4f88-b947-87ffeedbc449' },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'n' }] },
                {
                    JUMP: [
                        { NAME: '-done_043c1421-73fc-47e6-abf8-df6fcc7fea74' },
                        ['-done_043c1421-73fc-47e6-abf8-df6fcc7fea74'],
                    ],
                },
                { LABEL: '-newblock-cjump_4ddce186-c756-4f48-a7f3-56852d7f9e4a' },
                { LABEL: '-body_5e9319a3-ec22-4366-aaf6-1ee7f06f186e' },
                { MOVE: [{ LOCAL: 'n' }, { BINOP: ['PLUS', { LOCAL: 'n' }, { CONST: 1 }] }] },
                {
                    JUMP: [
                        { NAME: '-test_303a8443-90d2-4fc2-aedd-b7670c32a96d' },
                        ['-test_303a8443-90d2-4fc2-aedd-b7670c32a96d'],
                    ],
                },
                { LABEL: '-newblock-jump_d4f01582-b9a3-456d-816c-4d42accf8ce3' },
                {
                    JUMP: [
                        { NAME: '-while-done_94761299-2f79-4f88-b947-87ffeedbc449' },
                        ['-while-done_94761299-2f79-4f88-b947-87ffeedbc449'],
                    ],
                },
                { LABEL: '-done_043c1421-73fc-47e6-abf8-df6fcc7fea74' },
            ],
            frame: {
                label: '_tigermain_49ba3264-96dd-49b0-ad39-d72e4780dfd0',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
