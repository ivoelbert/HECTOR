import React from 'react';
import AceEditor, { IEditorProps } from 'react-ace';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { useCtrlKeys } from '../../hooks/useCtrlKeys';
import { baseCode } from '../../utils/baseCode';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/mode-golang';

interface CodeEditorProps {
    compileCode: (code: string) => void;
}

export const CodeEditor: React.FC<CodeEditorProps> = (props) => {
    const { compileCode } = props;

    const [code, setCode] = useLocalStorageState<string>('hector-code', baseCode);

    useCtrlKeys([13, 83], () => compileCode(code));

    return (
        <AceEditor
            mode="golang"
            theme="monokai"
            onChange={(newCode) => setCode(newCode)}
            value={code}
            name="code-editor"
            editorProps={{ $blockScrolling: true }}
            fontSize={14}
            width="100%"
            showPrintMargin={false}
            onLoad={(editor: IEditorProps) => editor.focus()}
            onBlur={() => compileCode(code)}
        />
    );
};
