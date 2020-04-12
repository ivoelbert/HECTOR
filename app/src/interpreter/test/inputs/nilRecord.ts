import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
	type rectype = {name : string, id : int}
	var b : rectype := nil
in
	b := nil; 0
end
*/

export const nilRecordInput: Frag[] = [
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_5179de28-737a-4405-a087-d428dfcbf0ee' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'b' }, { CONST: 0 }] },
                { MOVE: [{ LOCAL: 'b' }, { CONST: 0 }] },
                { MOVE: [{ GLOBAL: 'rv' }, { CONST: 0 }] },
                {
                    JUMP: [
                        { NAME: '-done_52f941e6-42e9-413a-b176-7e70541c9004' },
                        ['-done_52f941e6-42e9-413a-b176-7e70541c9004'],
                    ],
                },
                { LABEL: '-done_52f941e6-42e9-413a-b176-7e70541c9004' },
            ],
            frame: {
                label: '_tigermain_70753eff-f505-49e1-ae79-a1ab5161d881',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
