import React from 'react';
import AceEditor, { IEditorProps } from 'react-ace';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/mode-golang';

interface CodeEditorProps {
    code: string;
    setCode: React.Dispatch<React.SetStateAction<string>>;
    compileCode: () => void;
}

export const CodeEditor: React.FC<CodeEditorProps> = props => {
    const { code, setCode, compileCode } = props;

    return (
        <AceEditor
            mode="golang"
            theme="monokai"
            onChange={newCode => setCode(newCode)}
            value={code}
            name="code-editor"
            editorProps={{ $blockScrolling: true }}
            fontSize={14}
            width="100%"
            showPrintMargin={false}
            onLoad={(editor: IEditorProps) => editor.focus()}
            onBlur={compileCode}
        />
    );
};
