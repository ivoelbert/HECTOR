import { Frag } from '../../treeTypes';

/*
PROGRAM:

let
    function addone (n : int) : int =
        n + 1
in
    addone(41)
end
*/

export const addoneTestInput: Frag[] = [{"Proc":{"body":[{"LABEL":"-blockfirst_28b751d2-f761-4830-a0bb-73f39e21a50b"},{"MOVE":[{"MEM":{"BINOP":["PLUS",{"GLOBAL":"fp"},{"CONST":0}]}},{"LOCAL":"sl"}]},{"MOVE":[{"GLOBAL":"rv"},{"BINOP":["PLUS",{"LOCAL":"n"},{"CONST":1}]}]},{"JUMP":[{"NAME":"-done_bfd737f6-d301-41c2-b4fa-f429121a267f"},["-done_bfd737f6-d301-41c2-b4fa-f429121a267f"]]},{"LABEL":"-done_bfd737f6-d301-41c2-b4fa-f429121a267f"}],"frame":{"label":"addone_d1e3a51c-68a6-4543-8243-5966ec139ffc","formals":[["sl",true],["n",false]],"memindex":0}}},{"Proc":{"body":[{"LABEL":"-blockfirst_4758a32e-19aa-472e-80f1-0c4739f4b6c0"},{"MOVE":[{"MEM":{"BINOP":["PLUS",{"GLOBAL":"fp"},{"CONST":0}]}},{"LOCAL":"sl"}]},{"MOVE":[{"GLOBAL":"rv"},{"CALL":[{"NAME":"addone_d1e3a51c-68a6-4543-8243-5966ec139ffc"},[{"GLOBAL":"fp"},{"CONST":41}]]}]},{"JUMP":[{"NAME":"-done_a8a3971f-6fde-4dd9-868c-2b3486330203"},["-done_a8a3971f-6fde-4dd9-868c-2b3486330203"]]},{"LABEL":"-done_a8a3971f-6fde-4dd9-868c-2b3486330203"}],"frame":{"label":"_tigermain_16ab500c-7d3c-426b-bb67-10db7fe6b753","formals":[["sl",true]],"memindex":0}}}]