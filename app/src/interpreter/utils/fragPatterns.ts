import { Frag, FunFrag, StringFrag } from '../treeTypes';

export const isFunFrag = (fragment: Frag): fragment is FunFrag => {
    return 'Proc' in fragment;
};

export const isStringFrag = (fragment: Frag): fragment is StringFrag => {
    return 'ConstString' in fragment;
};
