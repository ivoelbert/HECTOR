import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
  type arrtype1 = array of int
  type rectype1 = {name : string, address : string, id : int, age : int}
  type arrtype2 = array of rectype1
  type rectype2 = {name : string, dates : arrtype1}

  type arrtype3 = array of string

  var arr1 := arrtype1 [10] of 0
  var arr2 := arrtype2 [5] of
  	 rectype1 {name="aname", address="somewhere", id=0, age=0}
  var arr3 : arrtype3 := arrtype3 [100] of ""

  var rec1 := rectype1 {name="Kapoios", address="Kapou", id=02432, age=44}
  var rec2 := rectype2 {name="Allos", dates=arrtype1 [3] of 1900}
in
  arr1[0] := 1;
  arr1[9] := 3;
  arr2[3].name := "kati";
  arr2[1].age := 23;
  arr3[34] := "sfd";

  rec1.name := "sdf";
  rec2.dates[0] := 2323;
  rec2.dates[2] := 2323; 0
end

*/

export const complexStructInput: Frag[] = [
    { ConstString: ['aname_ec0a0b52-6a1f-4d03-b764-bb02f5a702fd', 'aname'] },
    { ConstString: ['somewhere_a27de879-7bdc-49d0-bf93-5749e39521a8', 'somewhere'] },
    { ConstString: ['_cda2cdeb-2968-4357-88f6-a3b9f79a4f3e', ''] },
    { ConstString: ['Kapoios_fe9923bf-a141-436e-9a27-955a94f7f2ec', 'Kapoios'] },
    { ConstString: ['Kapou_e3091265-ce4d-409f-bbfe-49605dd4aa2e', 'Kapou'] },
    { ConstString: ['Allos_8d73bc57-d1a8-4902-a4bc-b99127dfb867', 'Allos'] },
    { ConstString: ['kati_46bf666d-ddca-485b-bb7f-4d7bc4e00519', 'kati'] },
    { ConstString: ['sfd_613ab1a4-34de-4e6c-96e7-165e190dc262', 'sfd'] },
    { ConstString: ['sdf_a9a88b1b-af02-46ac-a9ea-5766c28aed1f', 'sdf'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_39c0cc15-a1a4-46c7-8be8-fa520cecbdb4' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'arr1' },
                        { CALL: [{ NAME: '+alloc_array' }, [{ CONST: 10 }, { CONST: 0 }]] },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_08cb7cbf-416c-4ad2-8655-990226c2ba74' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 4 },
                                    { NAME: 'aname_ec0a0b52-6a1f-4d03-b764-bb02f5a702fd' },
                                    { NAME: 'somewhere_a27de879-7bdc-49d0-bf93-5749e39521a8' },
                                    { CONST: 0 },
                                    { CONST: 0 },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'arr2' },
                        {
                            CALL: [
                                { NAME: '+alloc_array' },
                                [
                                    { CONST: 5 },
                                    { LOCAL: '-reorder_call_08cb7cbf-416c-4ad2-8655-990226c2ba74' },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'arr3' },
                        {
                            CALL: [
                                { NAME: '+alloc_array' },
                                [{ CONST: 100 }, { NAME: '_cda2cdeb-2968-4357-88f6-a3b9f79a4f3e' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'rec1' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 4 },
                                    { NAME: 'Kapoios_fe9923bf-a141-436e-9a27-955a94f7f2ec' },
                                    { NAME: 'Kapou_e3091265-ce4d-409f-bbfe-49605dd4aa2e' },
                                    { CONST: 2432 },
                                    { CONST: 44 },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_1a6e1313-ce6a-4042-87f8-9a9a0ef9b120' },
                        { CALL: [{ NAME: '+alloc_array' }, [{ CONST: 3 }, { CONST: 1900 }]] },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'rec2' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 2 },
                                    { NAME: 'Allos_8d73bc57-d1a8-4902-a4bc-b99127dfb867' },
                                    { LOCAL: '-reorder_call_1a6e1313-ce6a-4042-87f8-9a9a0ef9b120' },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'arr1' }, { CONST: 0 }] } },
                        { CONST: 1 },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'arr1' }, { CONST: 9 }] } },
                        { CONST: 3 },
                    ],
                },
                {
                    MOVE: [
                        {
                            MEM: {
                                BINOP: [
                                    'PLUS',
                                    { MEM: { BINOP: ['PLUS', { LOCAL: 'arr2' }, { CONST: 3 }] } },
                                    { CONST: 0 },
                                ],
                            },
                        },
                        { NAME: 'kati_46bf666d-ddca-485b-bb7f-4d7bc4e00519' },
                    ],
                },
                {
                    MOVE: [
                        {
                            MEM: {
                                BINOP: [
                                    'PLUS',
                                    { MEM: { BINOP: ['PLUS', { LOCAL: 'arr2' }, { CONST: 1 }] } },
                                    { CONST: 3 },
                                ],
                            },
                        },
                        { CONST: 23 },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'arr3' }, { CONST: 34 }] } },
                        { NAME: 'sfd_613ab1a4-34de-4e6c-96e7-165e190dc262' },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'rec1' }, { CONST: 0 }] } },
                        { NAME: 'sdf_a9a88b1b-af02-46ac-a9ea-5766c28aed1f' },
                    ],
                },
                {
                    MOVE: [
                        {
                            MEM: {
                                BINOP: [
                                    'PLUS',
                                    { MEM: { BINOP: ['PLUS', { LOCAL: 'rec2' }, { CONST: 1 }] } },
                                    { CONST: 0 },
                                ],
                            },
                        },
                        { CONST: 2323 },
                    ],
                },
                {
                    MOVE: [
                        {
                            MEM: {
                                BINOP: [
                                    'PLUS',
                                    { MEM: { BINOP: ['PLUS', { LOCAL: 'rec2' }, { CONST: 1 }] } },
                                    { CONST: 2 },
                                ],
                            },
                        },
                        { CONST: 2323 },
                    ],
                },
                { MOVE: [{ GLOBAL: 'rv' }, { CONST: 0 }] },
                {
                    JUMP: [
                        { NAME: '-done_165f764b-fbeb-40b0-afeb-fe51472e91c4' },
                        ['-done_165f764b-fbeb-40b0-afeb-fe51472e91c4'],
                    ],
                },
                { LABEL: '-done_165f764b-fbeb-40b0-afeb-fe51472e91c4' },
            ],
            frame: {
                label: '_tigermain_4bfdf9da-53a9-4226-af69-ee6702c1b6fa',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
];
