import { Frag } from '../../treeTypes';

export const mergeInput: Frag[] = [
    { ConstString: ['0_77581e36-fe23-4ce0-bcfe-1c27f20c95b5', '0'] },
    { ConstString: ['9_c3438c9b-023e-4fa6-ad12-ca3a9a86e83a', '9'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_c018c327-e5f9-4026-b89d-ef3f5f84c34a' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_431db5d2-87ec-4c31-8207-9940d0be87ae' },
                        {
                            CALL: [
                                { NAME: 'ord' },
                                [{ NAME: '0_77581e36-fe23-4ce0-bcfe-1c27f20c95b5' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_5c07efe1-11d5-4915-852d-0945b6e6ca80' },
                        { LOCAL: '-reorder_call_431db5d2-87ec-4c31-8207-9940d0be87ae' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_4240dc05-2c28-48a8-b9dc-36af8a84de99' },
                        { CALL: [{ NAME: 'ord' }, [{ LOCAL: 's' }]] },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        {
                            BINOP: [
                                'ULE',
                                { LOCAL: '-reorder_5c07efe1-11d5-4915-852d-0945b6e6ca80' },
                                { LOCAL: '-reorder_call_4240dc05-2c28-48a8-b9dc-36af8a84de99' },
                            ],
                        },
                        { CONST: 1 },
                        '-then_60dec171-ead2-4816-9d44-387f9690e239',
                        '-else_a3528d78-f06d-4fe6-aca7-ddc148d6c936',
                    ],
                },
                { LABEL: '-else_a3528d78-f06d-4fe6-aca7-ddc148d6c936' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_5e52cf35-5c90-44cd-b044-9beecc936386' },
                        { CONST: 0 },
                    ],
                },
                { LABEL: '-join_0d48d9b5-6825-4b3a-b5ff-a12b7e5f46f9' },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        { LOCAL: '-ifresult_5e52cf35-5c90-44cd-b044-9beecc936386' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_797f47e7-9343-449b-a562-1ffeff73b9d8' },
                        ['-done_797f47e7-9343-449b-a562-1ffeff73b9d8'],
                    ],
                },
                { LABEL: '-newblock-cjump_a9d2d22d-bfc1-4739-924d-a26a23f981c5' },
                { LABEL: '-then_60dec171-ead2-4816-9d44-387f9690e239' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_7c130c9a-c863-4977-926d-5e566b0fc993' },
                        { CALL: [{ NAME: 'ord' }, [{ LOCAL: 's' }]] },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_6ffe04c4-359a-4f28-89a7-efbd8c749046' },
                        { LOCAL: '-reorder_call_7c130c9a-c863-4977-926d-5e566b0fc993' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_2d22354f-6a43-456f-9fc6-4dd9afde522b' },
                        {
                            CALL: [
                                { NAME: 'ord' },
                                [{ NAME: '9_c3438c9b-023e-4fa6-ad12-ca3a9a86e83a' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_5e52cf35-5c90-44cd-b044-9beecc936386' },
                        {
                            BINOP: [
                                'ULE',
                                { LOCAL: '-reorder_6ffe04c4-359a-4f28-89a7-efbd8c749046' },
                                { LOCAL: '-reorder_call_2d22354f-6a43-456f-9fc6-4dd9afde522b' },
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_0d48d9b5-6825-4b3a-b5ff-a12b7e5f46f9' },
                        ['-join_0d48d9b5-6825-4b3a-b5ff-a12b7e5f46f9'],
                    ],
                },
                { LABEL: '-newblock-jump_d20a4588-df93-46ed-81ef-37cdb1283d9b' },
                {
                    JUMP: [
                        { NAME: '-else_a3528d78-f06d-4fe6-aca7-ddc148d6c936' },
                        ['-else_a3528d78-f06d-4fe6-aca7-ddc148d6c936'],
                    ],
                },
                { LABEL: '-done_797f47e7-9343-449b-a562-1ffeff73b9d8' },
            ],
            frame: {
                label: 'isdigit_2d31744b-0e9b-4630-8608-d14dbace03b9',
                formals: [
                    ['sl', true],
                    ['s', false],
                ],
                memindex: 0,
            },
        },
    },
    { ConstString: ['_28dbf663-217b-4641-9df3-522ed888f7d4', ''] },
    { ConstString: ['\\n_476b7393-a63e-4ea6-91c6-78c2ab7c2635', '\\n'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_3fe4c800-7a9c-41c0-97de-b9095723d87e' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { LABEL: '-test_13b127b1-cba5-4cb8-a5c9-62d6b8521520' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_cc0a2be1-b9c5-4d3c-aeba-32401bbc116a' },
                        {
                            CALL: [
                                { NAME: '+str_equals' },
                                [
                                    {
                                        MEM: {
                                            BINOP: [
                                                'PLUS',
                                                {
                                                    MEM: {
                                                        BINOP: [
                                                            'PLUS',
                                                            {
                                                                MEM: {
                                                                    BINOP: [
                                                                        'PLUS',
                                                                        { GLOBAL: 'fp' },
                                                                        { CONST: 0 },
                                                                    ],
                                                                },
                                                            },
                                                            { CONST: 0 },
                                                        ],
                                                    },
                                                },
                                                { CONST: 1 },
                                            ],
                                        },
                                    },
                                    { NAME: '_28dbf663-217b-4641-9df3-522ed888f7d4' },
                                ],
                            ],
                        },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { LOCAL: '-reorder_call_cc0a2be1-b9c5-4d3c-aeba-32401bbc116a' },
                        { CONST: 1 },
                        '-then_3b50dfe9-44e9-4a02-9483-0257dd738a66',
                        '-else_05e27e97-6f61-47ba-8a5c-fdbd1965261c',
                    ],
                },
                { LABEL: '-else_05e27e97-6f61-47ba-8a5c-fdbd1965261c' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_1811eb66-4611-45fe-9b09-1395942bb4f4' },
                        {
                            CALL: [
                                { NAME: '+str_equals' },
                                [
                                    {
                                        MEM: {
                                            BINOP: [
                                                'PLUS',
                                                {
                                                    MEM: {
                                                        BINOP: [
                                                            'PLUS',
                                                            {
                                                                MEM: {
                                                                    BINOP: [
                                                                        'PLUS',
                                                                        { GLOBAL: 'fp' },
                                                                        { CONST: 0 },
                                                                    ],
                                                                },
                                                            },
                                                            { CONST: 0 },
                                                        ],
                                                    },
                                                },
                                                { CONST: 1 },
                                            ],
                                        },
                                    },
                                    { NAME: '\\n_476b7393-a63e-4ea6-91c6-78c2ab7c2635' },
                                ],
                            ],
                        },
                    ],
                },
                { LABEL: '-join_e51ccbc1-de93-4f7a-b267-74606759fbf0' },
                {
                    CJUMP: [
                        'GE',
                        { LOCAL: '-ifresult_1811eb66-4611-45fe-9b09-1395942bb4f4' },
                        { CONST: 1 },
                        '-body_13dbf132-18fc-45ea-983c-812b2e165a4f',
                        '-while-done_02d9e3b4-2f6d-476d-aedc-87109f0c8d39',
                    ],
                },
                { LABEL: '-while-done_02d9e3b4-2f6d-476d-aedc-87109f0c8d39' },
                {
                    JUMP: [
                        { NAME: '-done_ebf1373f-fc04-4711-982d-fb80f1d61cbd' },
                        ['-done_ebf1373f-fc04-4711-982d-fb80f1d61cbd'],
                    ],
                },
                { LABEL: '-newblock-cjump_cd3160f6-919d-4024-919d-3b3a7585e2f7' },
                { LABEL: '-then_3b50dfe9-44e9-4a02-9483-0257dd738a66' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_1811eb66-4611-45fe-9b09-1395942bb4f4' },
                        { CONST: 1 },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_e51ccbc1-de93-4f7a-b267-74606759fbf0' },
                        ['-join_e51ccbc1-de93-4f7a-b267-74606759fbf0'],
                    ],
                },
                { LABEL: '-newblock-jump_3e4b6a7e-77cb-4c86-9ac8-b42cd73962ef' },
                {
                    JUMP: [
                        { NAME: '-else_05e27e97-6f61-47ba-8a5c-fdbd1965261c' },
                        ['-else_05e27e97-6f61-47ba-8a5c-fdbd1965261c'],
                    ],
                },
                { LABEL: '-newblock-cjump_e7ffdb62-7180-4d0b-88f3-ad712effee76' },
                { LABEL: '-body_13dbf132-18fc-45ea-983c-812b2e165a4f' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_8fea66a1-f07b-47ce-a950-3e12ce26eeb4' },
                        { CALL: [{ NAME: 'getchar' }, []] },
                    ],
                },
                {
                    MOVE: [
                        {
                            MEM: {
                                BINOP: [
                                    'PLUS',
                                    {
                                        MEM: {
                                            BINOP: [
                                                'PLUS',
                                                {
                                                    MEM: {
                                                        BINOP: [
                                                            'PLUS',
                                                            { GLOBAL: 'fp' },
                                                            { CONST: 0 },
                                                        ],
                                                    },
                                                },
                                                { CONST: 0 },
                                            ],
                                        },
                                    },
                                    { CONST: 1 },
                                ],
                            },
                        },
                        { LOCAL: '-reorder_call_8fea66a1-f07b-47ce-a950-3e12ce26eeb4' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-test_13b127b1-cba5-4cb8-a5c9-62d6b8521520' },
                        ['-test_13b127b1-cba5-4cb8-a5c9-62d6b8521520'],
                    ],
                },
                { LABEL: '-newblock-jump_28119591-cd4e-4980-8579-b5c1b3478ef0' },
                {
                    JUMP: [
                        { NAME: '-while-done_02d9e3b4-2f6d-476d-aedc-87109f0c8d39' },
                        ['-while-done_02d9e3b4-2f6d-476d-aedc-87109f0c8d39'],
                    ],
                },
                { LABEL: '-done_ebf1373f-fc04-4711-982d-fb80f1d61cbd' },
            ],
            frame: {
                label: 'skipto_529ccd02-d506-44cf-938f-b7b55c9c96d1',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
    { ConstString: ['0_664b99d3-1640-4f64-a20d-2d86ccd09c48', '0'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_106e6ecf-b3f5-43fb-94e3-d7763f23c7fc' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                { MOVE: [{ LOCAL: 'i' }, { CONST: 0 }] },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'skipto_529ccd02-d506-44cf-938f-b7b55c9c96d1' },
                            [{ GLOBAL: 'fp' }],
                        ],
                    },
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_b31fa4a6-876e-4795-a15c-1b07aaf57dd3' },
                        {
                            CALL: [
                                { NAME: 'isdigit_2d31744b-0e9b-4630-8608-d14dbace03b9' },
                                [
                                    { GLOBAL: 'fp' },
                                    {
                                        MEM: {
                                            BINOP: [
                                                'PLUS',
                                                {
                                                    MEM: {
                                                        BINOP: [
                                                            'PLUS',
                                                            { GLOBAL: 'fp' },
                                                            { CONST: 0 },
                                                        ],
                                                    },
                                                },
                                                { CONST: 1 },
                                            ],
                                        },
                                    },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'any' }, { CONST: 0 }] } },
                        { LOCAL: '-reorder_call_b31fa4a6-876e-4795-a15c-1b07aaf57dd3' },
                    ],
                },
                { LABEL: '-test_1effb048-cf99-4b30-9509-c41e346c0795' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_a24a2d13-6f83-4ab1-b4ab-9dda9e1e53a3' },
                        {
                            CALL: [
                                { NAME: 'isdigit_2d31744b-0e9b-4630-8608-d14dbace03b9' },
                                [
                                    { GLOBAL: 'fp' },
                                    {
                                        MEM: {
                                            BINOP: [
                                                'PLUS',
                                                {
                                                    MEM: {
                                                        BINOP: [
                                                            'PLUS',
                                                            { GLOBAL: 'fp' },
                                                            { CONST: 0 },
                                                        ],
                                                    },
                                                },
                                                { CONST: 1 },
                                            ],
                                        },
                                    },
                                ],
                            ],
                        },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { LOCAL: '-reorder_call_a24a2d13-6f83-4ab1-b4ab-9dda9e1e53a3' },
                        { CONST: 1 },
                        '-body_861ab750-583c-4a1b-a6cd-570c380df5e1',
                        '-while-done_7365f6a8-719e-4870-9789-6ce35cda05f7',
                    ],
                },
                { LABEL: '-while-done_7365f6a8-719e-4870-9789-6ce35cda05f7' },
                { MOVE: [{ GLOBAL: 'rv' }, { LOCAL: 'i' }] },
                {
                    JUMP: [
                        { NAME: '-done_b5c3ed9b-7d88-49ab-be9b-04ff6215fb5e' },
                        ['-done_b5c3ed9b-7d88-49ab-be9b-04ff6215fb5e'],
                    ],
                },
                { LABEL: '-newblock-cjump_e88db28c-a7b8-4bff-a73a-d0253dbf5b03' },
                { LABEL: '-body_861ab750-583c-4a1b-a6cd-570c380df5e1' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_ae248155-5acd-45bb-a0c9-4fe53c4ede23' },
                        { BINOP: ['MUL', { LOCAL: 'i' }, { CONST: 10 }] },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_2097d48e-a950-413c-b762-a082744cfa6a' },
                        {
                            CALL: [
                                { NAME: 'ord' },
                                [
                                    {
                                        MEM: {
                                            BINOP: [
                                                'PLUS',
                                                {
                                                    MEM: {
                                                        BINOP: [
                                                            'PLUS',
                                                            { GLOBAL: 'fp' },
                                                            { CONST: 0 },
                                                        ],
                                                    },
                                                },
                                                { CONST: 1 },
                                            ],
                                        },
                                    },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_8f4d834a-e44d-46d0-9552-c7258c611274' },
                        {
                            BINOP: [
                                'PLUS',
                                { LOCAL: '-reorder_ae248155-5acd-45bb-a0c9-4fe53c4ede23' },
                                { LOCAL: '-reorder_call_2097d48e-a950-413c-b762-a082744cfa6a' },
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_0c317bf7-0b72-4aa4-bb82-bafbc1c208e4' },
                        {
                            CALL: [
                                { NAME: 'ord' },
                                [{ NAME: '0_664b99d3-1640-4f64-a20d-2d86ccd09c48' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'i' },
                        {
                            BINOP: [
                                'MINUS',
                                { LOCAL: '-reorder_8f4d834a-e44d-46d0-9552-c7258c611274' },
                                { LOCAL: '-reorder_call_0c317bf7-0b72-4aa4-bb82-bafbc1c208e4' },
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_54717043-fec7-4b1b-b6ff-2deb7a939c55' },
                        { CALL: [{ NAME: 'getchar' }, []] },
                    ],
                },
                {
                    MOVE: [
                        {
                            MEM: {
                                BINOP: [
                                    'PLUS',
                                    { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                                    { CONST: 1 },
                                ],
                            },
                        },
                        { LOCAL: '-reorder_call_54717043-fec7-4b1b-b6ff-2deb7a939c55' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-test_1effb048-cf99-4b30-9509-c41e346c0795' },
                        ['-test_1effb048-cf99-4b30-9509-c41e346c0795'],
                    ],
                },
                { LABEL: '-newblock-jump_2a5dd7c4-96e2-4d13-961b-292cf9f338a9' },
                {
                    JUMP: [
                        { NAME: '-while-done_7365f6a8-719e-4870-9789-6ce35cda05f7' },
                        ['-while-done_7365f6a8-719e-4870-9789-6ce35cda05f7'],
                    ],
                },
                { LABEL: '-done_b5c3ed9b-7d88-49ab-be9b-04ff6215fb5e' },
            ],
            frame: {
                label: 'readint_f7f8bd90-ff15-48d1-9b62-f4315722d02e',
                formals: [
                    ['sl', true],
                    ['any', false],
                ],
                memindex: 0,
            },
        },
    },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_cbbe2db0-18ed-44ad-bbe6-d54c4cb04508' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'any' },
                        { CALL: [{ NAME: '+alloc_record' }, [{ CONST: 1 }, { CONST: 0 }]] },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'i' },
                        {
                            CALL: [
                                { NAME: 'readint_f7f8bd90-ff15-48d1-9b62-f4315722d02e' },
                                [{ GLOBAL: 'fp' }, { LOCAL: 'any' }],
                            ],
                        },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'any' }, { CONST: 0 }] } },
                        { CONST: 1 },
                        '-then_24791f3f-78b9-4852-82ba-92b84180863b',
                        '-else_d4f229e2-9ccd-409d-9006-127bef5f9147',
                    ],
                },
                { LABEL: '-else_d4f229e2-9ccd-409d-9006-127bef5f9147' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_ba8e20a9-1bea-40a6-b243-ab8c1b8b4710' },
                        { CONST: 0 },
                    ],
                },
                { LABEL: '-join_2b35a25f-8fe7-4d59-92e5-4e17f061edac' },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        { LOCAL: '-ifresult_ba8e20a9-1bea-40a6-b243-ab8c1b8b4710' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_31030fbe-10bf-4c9d-a009-80f9f37766c3' },
                        ['-done_31030fbe-10bf-4c9d-a009-80f9f37766c3'],
                    ],
                },
                { LABEL: '-newblock-cjump_d2667bbb-746a-40e7-9d56-1bbf2eef7c81' },
                { LABEL: '-then_24791f3f-78b9-4852-82ba-92b84180863b' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_0a321aa5-cd21-44bf-af20-64353ac14cd7' },
                        { LOCAL: 'i' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_07ab18ee-ae4d-4d1f-b24e-8bb3f2cf030d' },
                        {
                            CALL: [
                                { NAME: 'readlist_9ccd14a4-e65c-43c0-ab0a-3e364ceef9f2' },
                                [{ GLOBAL: 'fp' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_ba8e20a9-1bea-40a6-b243-ab8c1b8b4710' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 2 },
                                    { LOCAL: '-reorder_0a321aa5-cd21-44bf-af20-64353ac14cd7' },
                                    { LOCAL: '-reorder_call_07ab18ee-ae4d-4d1f-b24e-8bb3f2cf030d' },
                                ],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_2b35a25f-8fe7-4d59-92e5-4e17f061edac' },
                        ['-join_2b35a25f-8fe7-4d59-92e5-4e17f061edac'],
                    ],
                },
                { LABEL: '-newblock-jump_388c2ae0-be57-4149-a137-78494d22ec75' },
                {
                    JUMP: [
                        { NAME: '-else_d4f229e2-9ccd-409d-9006-127bef5f9147' },
                        ['-else_d4f229e2-9ccd-409d-9006-127bef5f9147'],
                    ],
                },
                { LABEL: '-done_31030fbe-10bf-4c9d-a009-80f9f37766c3' },
            ],
            frame: {
                label: 'readlist_9ccd14a4-e65c-43c0-ab0a-3e364ceef9f2',
                formals: [['sl', true]],
                memindex: 0,
            },
        },
    },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_81bab688-d166-42f0-ba8d-86c043384c1d' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { CONST: 0 },
                        { CONST: 1 },
                        '-then_f6bc8ce5-0158-4af2-95f2-7a061131b9b2',
                        '-else_1de71c65-91a6-479e-a02c-f7978a87c529',
                    ],
                },
                { LABEL: '-else_1de71c65-91a6-479e-a02c-f7978a87c529' },
                {
                    CJUMP: [
                        'GE',
                        { CONST: 0 },
                        { CONST: 1 },
                        '-then_75e5ce47-c07c-481b-ae21-738970cbe61e',
                        '-else_55c2ad40-f768-4e5e-b032-c3ac0f05d198',
                    ],
                },
                { LABEL: '-else_55c2ad40-f768-4e5e-b032-c3ac0f05d198' },
                {
                    CJUMP: [
                        'GE',
                        {
                            BINOP: [
                                'ULT',
                                { MEM: { BINOP: ['PLUS', { LOCAL: 'a' }, { CONST: 0 }] } },
                                { MEM: { BINOP: ['PLUS', { LOCAL: 'b' }, { CONST: 0 }] } },
                            ],
                        },
                        { CONST: 1 },
                        '-then_30f25e0d-39aa-4825-b5e9-572ca53fd5f8',
                        '-else_a727ba28-30d4-463e-b671-52560aed336f',
                    ],
                },
                { LABEL: '-else_a727ba28-30d4-463e-b671-52560aed336f' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_58a537e4-f728-4297-82f0-e0993839ce16' },
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'b' }, { CONST: 0 }] } },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_805c3024-d60e-4dfa-bc94-12dc8019d175' },
                        {
                            CALL: [
                                { NAME: 'merge_44751c89-0c8c-4cca-8087-f32312c7a076' },
                                [
                                    { GLOBAL: 'fp' },
                                    { LOCAL: 'a' },
                                    { MEM: { BINOP: ['PLUS', { LOCAL: 'b' }, { CONST: 1 }] } },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_9ee9e536-904e-494c-b7d6-b1b61f704005' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 2 },
                                    { LOCAL: '-reorder_58a537e4-f728-4297-82f0-e0993839ce16' },
                                    { LOCAL: '-reorder_call_805c3024-d60e-4dfa-bc94-12dc8019d175' },
                                ],
                            ],
                        },
                    ],
                },
                { LABEL: '-join_0c82f179-72c8-4211-9298-577f1d0ee185' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_02fe396d-9df5-46e2-97d9-a9f129d21b91' },
                        { LOCAL: '-ifresult_9ee9e536-904e-494c-b7d6-b1b61f704005' },
                    ],
                },
                { LABEL: '-join_f19cc035-d56c-4da9-a41f-6c41a61c9330' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_0e910a1f-e1d2-44a9-ba5c-805f92759b07' },
                        { LOCAL: '-ifresult_02fe396d-9df5-46e2-97d9-a9f129d21b91' },
                    ],
                },
                { LABEL: '-join_957e3a68-f3d1-4720-a1dc-d3f316a6e87f' },
                {
                    MOVE: [
                        { GLOBAL: 'rv' },
                        { LOCAL: '-ifresult_0e910a1f-e1d2-44a9-ba5c-805f92759b07' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-done_ae3d9e74-343a-4473-936d-9d0beae41c2c' },
                        ['-done_ae3d9e74-343a-4473-936d-9d0beae41c2c'],
                    ],
                },
                { LABEL: '-newblock-cjump_9dd1e198-d805-41d4-bf57-ef08b4da6245' },
                { LABEL: '-then_f6bc8ce5-0158-4af2-95f2-7a061131b9b2' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_0e910a1f-e1d2-44a9-ba5c-805f92759b07' },
                        { LOCAL: 'b' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_957e3a68-f3d1-4720-a1dc-d3f316a6e87f' },
                        ['-join_957e3a68-f3d1-4720-a1dc-d3f316a6e87f'],
                    ],
                },
                { LABEL: '-newblock-jump_b7192d2d-c8a6-4d9d-a068-01a67415b4f7' },
                {
                    JUMP: [
                        { NAME: '-else_1de71c65-91a6-479e-a02c-f7978a87c529' },
                        ['-else_1de71c65-91a6-479e-a02c-f7978a87c529'],
                    ],
                },
                { LABEL: '-newblock-cjump_3003f716-ef56-4461-bdd0-6c95fcb368ab' },
                { LABEL: '-then_75e5ce47-c07c-481b-ae21-738970cbe61e' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_02fe396d-9df5-46e2-97d9-a9f129d21b91' },
                        { LOCAL: 'a' },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_f19cc035-d56c-4da9-a41f-6c41a61c9330' },
                        ['-join_f19cc035-d56c-4da9-a41f-6c41a61c9330'],
                    ],
                },
                { LABEL: '-newblock-jump_d5176d17-9db6-4234-893f-e260063904ab' },
                {
                    JUMP: [
                        { NAME: '-else_55c2ad40-f768-4e5e-b032-c3ac0f05d198' },
                        ['-else_55c2ad40-f768-4e5e-b032-c3ac0f05d198'],
                    ],
                },
                { LABEL: '-newblock-cjump_b152922c-fa0b-4906-aaa7-146a00e1b9a9' },
                { LABEL: '-then_30f25e0d-39aa-4825-b5e9-572ca53fd5f8' },
                {
                    MOVE: [
                        { LOCAL: '-reorder_0e29a41f-64a4-44a7-bc1b-b48f1ea4c082' },
                        { MEM: { BINOP: ['PLUS', { LOCAL: 'a' }, { CONST: 0 }] } },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_ae5a7767-6138-43a2-aa48-4abd38dca5c3' },
                        {
                            CALL: [
                                { NAME: 'merge_44751c89-0c8c-4cca-8087-f32312c7a076' },
                                [
                                    { GLOBAL: 'fp' },
                                    { MEM: { BINOP: ['PLUS', { LOCAL: 'a' }, { CONST: 1 }] } },
                                    { LOCAL: 'b' },
                                ],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_9ee9e536-904e-494c-b7d6-b1b61f704005' },
                        {
                            CALL: [
                                { NAME: '+alloc_record' },
                                [
                                    { CONST: 2 },
                                    { LOCAL: '-reorder_0e29a41f-64a4-44a7-bc1b-b48f1ea4c082' },
                                    { LOCAL: '-reorder_call_ae5a7767-6138-43a2-aa48-4abd38dca5c3' },
                                ],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_0c82f179-72c8-4211-9298-577f1d0ee185' },
                        ['-join_0c82f179-72c8-4211-9298-577f1d0ee185'],
                    ],
                },
                { LABEL: '-newblock-jump_5ceac724-3cd2-4d9b-a800-cf5b790a9f81' },
                {
                    JUMP: [
                        { NAME: '-else_a727ba28-30d4-463e-b671-52560aed336f' },
                        ['-else_a727ba28-30d4-463e-b671-52560aed336f'],
                    ],
                },
                { LABEL: '-done_ae3d9e74-343a-4473-936d-9d0beae41c2c' },
            ],
            frame: {
                label: 'merge_44751c89-0c8c-4cca-8087-f32312c7a076',
                formals: [
                    ['sl', true],
                    ['a', false],
                    ['b', false],
                ],
                memindex: 0,
            },
        },
    },
    { ConstString: ['0_d5cd2b30-2889-4116-a25e-22bcd341528b', '0'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_d7a9fdcb-8015-481e-8f9b-fdb68a36c46f' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { BINOP: ['UGT', { LOCAL: 'i' }, { CONST: 0 }] },
                        { CONST: 1 },
                        '-then_fbbf8405-5dd5-4e19-beb3-91f053a43b97',
                        '-else_fa5030da-c9a0-47b3-bcbc-93172e263a71',
                    ],
                },
                { LABEL: '-else_fa5030da-c9a0-47b3-bcbc-93172e263a71' },
                {
                    JUMP: [
                        { NAME: '-done_23a79517-0ffa-4f5e-91ab-ccf2bcfc8000' },
                        ['-done_23a79517-0ffa-4f5e-91ab-ccf2bcfc8000'],
                    ],
                },
                { LABEL: '-newblock-cjump_d9d543c8-aca3-43e0-8305-d599674a7f28' },
                { LABEL: '-then_fbbf8405-5dd5-4e19-beb3-91f053a43b97' },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'f_88bafe39-3bd2-4d16-aee4-354db78a931b' },
                            [{ GLOBAL: 'fp' }, { BINOP: ['DIV', { LOCAL: 'i' }, { CONST: 10 }] }],
                        ],
                    },
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_1161ac8c-bd3a-4d92-ac37-9e4b67d3b0d5' },
                        {
                            BINOP: [
                                'MINUS',
                                { LOCAL: 'i' },
                                {
                                    BINOP: [
                                        'MUL',
                                        { BINOP: ['DIV', { LOCAL: 'i' }, { CONST: 10 }] },
                                        { CONST: 10 },
                                    ],
                                },
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_9f02df99-93d5-45a6-a9b5-e59be7a8326c' },
                        {
                            CALL: [
                                { NAME: 'ord' },
                                [{ NAME: '0_d5cd2b30-2889-4116-a25e-22bcd341528b' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_79e222df-b2a0-45e0-856d-82ab84f64f8c' },
                        {
                            CALL: [
                                { NAME: 'chr' },
                                [
                                    {
                                        BINOP: [
                                            'PLUS',
                                            {
                                                LOCAL:
                                                    '-reorder_1161ac8c-bd3a-4d92-ac37-9e4b67d3b0d5',
                                            },
                                            {
                                                LOCAL:
                                                    '-reorder_call_9f02df99-93d5-45a6-a9b5-e59be7a8326c',
                                            },
                                        ],
                                    },
                                ],
                            ],
                        },
                    ],
                },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'print' },
                            [{ LOCAL: '-reorder_call_79e222df-b2a0-45e0-856d-82ab84f64f8c' }],
                        ],
                    },
                },
                {
                    JUMP: [
                        { NAME: '-else_fa5030da-c9a0-47b3-bcbc-93172e263a71' },
                        ['-else_fa5030da-c9a0-47b3-bcbc-93172e263a71'],
                    ],
                },
                { LABEL: '-done_23a79517-0ffa-4f5e-91ab-ccf2bcfc8000' },
            ],
            frame: {
                label: 'f_88bafe39-3bd2-4d16-aee4-354db78a931b',
                formals: [
                    ['sl', true],
                    ['i', false],
                ],
                memindex: 0,
            },
        },
    },
    { ConstString: ['-_c29b21ea-387b-44fd-b8e3-ad1eaa3314fb', '-'] },
    { ConstString: ['0_ad189d71-2b10-479f-a7d7-016b109754cd', '0'] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_8a8bc90c-ed70-489e-85e9-39a28ff916f6' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { BINOP: ['ULT', { LOCAL: 'i' }, { CONST: 0 }] },
                        { CONST: 1 },
                        '-then_0ed5da04-9a44-4de8-80af-ad3f489f5def',
                        '-else_173afba6-cbe0-44c6-8e63-a29b8e51ecf5',
                    ],
                },
                { LABEL: '-else_173afba6-cbe0-44c6-8e63-a29b8e51ecf5' },
                {
                    CJUMP: [
                        'GE',
                        { BINOP: ['UGT', { LOCAL: 'i' }, { CONST: 0 }] },
                        { CONST: 1 },
                        '-then_5ecfa0ca-af61-4207-a554-1a9bdd6ebf4e',
                        '-else_770a84d4-5352-4d52-94e9-e523c28987da',
                    ],
                },
                { LABEL: '-else_770a84d4-5352-4d52-94e9-e523c28987da' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_3f112cfe-3052-48e4-b875-a20dca671f75' },
                        {
                            CALL: [
                                { NAME: 'print' },
                                [{ NAME: '0_ad189d71-2b10-479f-a7d7-016b109754cd' }],
                            ],
                        },
                    ],
                },
                { LABEL: '-join_bb1241d2-7ff0-4f3d-b5b6-317c8d421867' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_a8d318a2-8784-4252-8c87-dd6412960b4f' },
                        { LOCAL: '-ifresult_3f112cfe-3052-48e4-b875-a20dca671f75' },
                    ],
                },
                { LABEL: '-join_6ff56060-4567-4e99-9519-1ba5a71c9dbb' },
                { EXP: { LOCAL: '-ifresult_a8d318a2-8784-4252-8c87-dd6412960b4f' } },
                {
                    JUMP: [
                        { NAME: '-done_0b8e727f-a79d-4768-9d71-054e4933c994' },
                        ['-done_0b8e727f-a79d-4768-9d71-054e4933c994'],
                    ],
                },
                { LABEL: '-newblock-cjump_b2acf42f-3cb1-4203-8960-a5cef62b9a24' },
                { LABEL: '-then_0ed5da04-9a44-4de8-80af-ad3f489f5def' },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'print' },
                            [{ NAME: '-_c29b21ea-387b-44fd-b8e3-ad1eaa3314fb' }],
                        ],
                    },
                },
                { EXP: { CONST: 0 } },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_a8d318a2-8784-4252-8c87-dd6412960b4f' },
                        {
                            CALL: [
                                { NAME: 'f_88bafe39-3bd2-4d16-aee4-354db78a931b' },
                                [
                                    { GLOBAL: 'fp' },
                                    { BINOP: ['MINUS', { CONST: 0 }, { LOCAL: 'i' }] },
                                ],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_6ff56060-4567-4e99-9519-1ba5a71c9dbb' },
                        ['-join_6ff56060-4567-4e99-9519-1ba5a71c9dbb'],
                    ],
                },
                { LABEL: '-newblock-jump_4b7215b0-8384-45d2-9885-de2ea565b945' },
                {
                    JUMP: [
                        { NAME: '-else_173afba6-cbe0-44c6-8e63-a29b8e51ecf5' },
                        ['-else_173afba6-cbe0-44c6-8e63-a29b8e51ecf5'],
                    ],
                },
                { LABEL: '-newblock-cjump_3b7f6f20-dc95-4a1c-9e28-74d8ab02a3b2' },
                { LABEL: '-then_5ecfa0ca-af61-4207-a554-1a9bdd6ebf4e' },
                {
                    MOVE: [
                        { LOCAL: '-ifresult_3f112cfe-3052-48e4-b875-a20dca671f75' },
                        {
                            CALL: [
                                { NAME: 'f_88bafe39-3bd2-4d16-aee4-354db78a931b' },
                                [{ GLOBAL: 'fp' }, { LOCAL: 'i' }],
                            ],
                        },
                    ],
                },
                {
                    JUMP: [
                        { NAME: '-join_bb1241d2-7ff0-4f3d-b5b6-317c8d421867' },
                        ['-join_bb1241d2-7ff0-4f3d-b5b6-317c8d421867'],
                    ],
                },
                { LABEL: '-newblock-jump_bc77b7b2-6586-412f-86dc-13697f9cd139' },
                {
                    JUMP: [
                        { NAME: '-else_770a84d4-5352-4d52-94e9-e523c28987da' },
                        ['-else_770a84d4-5352-4d52-94e9-e523c28987da'],
                    ],
                },
                { LABEL: '-done_0b8e727f-a79d-4768-9d71-054e4933c994' },
            ],
            frame: {
                label: 'printint_dc3abbc3-18fc-42e6-b7a6-1cbc538dcd2b',
                formals: [
                    ['sl', true],
                    ['i', false],
                ],
                memindex: 0,
            },
        },
    },
    { ConstString: ['\\n_a34d8165-e579-46af-922a-ecd68a2fc77a', '\\n'] },
    { ConstString: [' _e6956a65-ed92-4294-80e7-39017322d7c9', ' '] },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_44216ffe-913f-4ec9-8663-8459e09946de' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    CJUMP: [
                        'GE',
                        { CONST: 0 },
                        { CONST: 1 },
                        '-then_b01b8b46-56eb-4f25-93b3-4690b198832e',
                        '-else_409061d7-3b85-476f-8e82-8bb75d09d4ef',
                    ],
                },
                { LABEL: '-else_409061d7-3b85-476f-8e82-8bb75d09d4ef' },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'printint_dc3abbc3-18fc-42e6-b7a6-1cbc538dcd2b' },
                            [
                                { GLOBAL: 'fp' },
                                { MEM: { BINOP: ['PLUS', { LOCAL: 'l' }, { CONST: 0 }] } },
                            ],
                        ],
                    },
                },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'print' },
                            [{ NAME: ' _e6956a65-ed92-4294-80e7-39017322d7c9' }],
                        ],
                    },
                },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'printlist_05231958-3384-4d6e-8b9d-25895f62543e' },
                            [
                                { GLOBAL: 'fp' },
                                { MEM: { BINOP: ['PLUS', { LOCAL: 'l' }, { CONST: 1 }] } },
                            ],
                        ],
                    },
                },
                { LABEL: '-join_9555886c-6579-4b14-a5c7-eb327f00f0be' },
                {
                    JUMP: [
                        { NAME: '-done_6ff17a2d-bdd4-4bb5-b266-b33bfd5095ab' },
                        ['-done_6ff17a2d-bdd4-4bb5-b266-b33bfd5095ab'],
                    ],
                },
                { LABEL: '-newblock-cjump_043b6db8-d0f7-4611-9176-8307d8b542ce' },
                { LABEL: '-then_b01b8b46-56eb-4f25-93b3-4690b198832e' },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'print' },
                            [{ NAME: '\\n_a34d8165-e579-46af-922a-ecd68a2fc77a' }],
                        ],
                    },
                },
                {
                    JUMP: [
                        { NAME: '-join_9555886c-6579-4b14-a5c7-eb327f00f0be' },
                        ['-join_9555886c-6579-4b14-a5c7-eb327f00f0be'],
                    ],
                },
                { LABEL: '-newblock-jump_be7f4d30-c7cd-45af-b0a8-91c048b0b21c' },
                {
                    JUMP: [
                        { NAME: '-else_409061d7-3b85-476f-8e82-8bb75d09d4ef' },
                        ['-else_409061d7-3b85-476f-8e82-8bb75d09d4ef'],
                    ],
                },
                { LABEL: '-done_6ff17a2d-bdd4-4bb5-b266-b33bfd5095ab' },
            ],
            frame: {
                label: 'printlist_05231958-3384-4d6e-8b9d-25895f62543e',
                formals: [
                    ['sl', true],
                    ['l', false],
                ],
                memindex: 0,
            },
        },
    },
    {
        Proc: {
            body: [
                { LABEL: '-blockfirst_7a1679cf-f4fc-4c64-98c4-b9a946f272c3' },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 0 }] } },
                        { LOCAL: 'sl' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_27d3d6be-bc8f-497d-ac2d-ed2e5b15eec7' },
                        { CALL: [{ NAME: 'getchar' }, []] },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 1 }] } },
                        { LOCAL: '-reorder_call_27d3d6be-bc8f-497d-ac2d-ed2e5b15eec7' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'list1' },
                        {
                            CALL: [
                                { NAME: 'readlist_9ccd14a4-e65c-43c0-ab0a-3e364ceef9f2' },
                                [{ GLOBAL: 'fp' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_5bf2c986-9c60-401b-8697-daac23f83344' },
                        { CALL: [{ NAME: 'getchar' }, []] },
                    ],
                },
                {
                    MOVE: [
                        { MEM: { BINOP: ['PLUS', { GLOBAL: 'fp' }, { CONST: 1 }] } },
                        { LOCAL: '-reorder_call_5bf2c986-9c60-401b-8697-daac23f83344' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: 'list2' },
                        {
                            CALL: [
                                { NAME: 'readlist_9ccd14a4-e65c-43c0-ab0a-3e364ceef9f2' },
                                [{ GLOBAL: 'fp' }],
                            ],
                        },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_94ea42bf-cb18-4e07-b9d9-cdf43e152d4a' },
                        { GLOBAL: 'fp' },
                    ],
                },
                {
                    MOVE: [
                        { LOCAL: '-reorder_call_11373bf7-1167-4e55-8397-acfc2168fafb' },
                        {
                            CALL: [
                                { NAME: 'merge_44751c89-0c8c-4cca-8087-f32312c7a076' },
                                [{ GLOBAL: 'fp' }, { LOCAL: 'list1' }, { LOCAL: 'list2' }],
                            ],
                        },
                    ],
                },
                {
                    EXP: {
                        CALL: [
                            { NAME: 'printlist_05231958-3384-4d6e-8b9d-25895f62543e' },
                            [
                                { LOCAL: '-reorder_94ea42bf-cb18-4e07-b9d9-cdf43e152d4a' },
                                { LOCAL: '-reorder_call_11373bf7-1167-4e55-8397-acfc2168fafb' },
                            ],
                        ],
                    },
                },
                { MOVE: [{ GLOBAL: 'rv' }, { CONST: 0 }] },
                {
                    JUMP: [
                        { NAME: '-done_8b7b5542-bc11-4d24-a905-900eaf0f68d1' },
                        ['-done_8b7b5542-bc11-4d24-a905-900eaf0f68d1'],
                    ],
                },
                { LABEL: '-done_8b7b5542-bc11-4d24-a905-900eaf0f68d1' },
            ],
            frame: {
                label: '_tigermain_3c9e0f33-7770-4d7d-9581-acf5131c36e7',
                formals: [['sl', true]],
                memindex: 1,
            },
        },
    },
];
