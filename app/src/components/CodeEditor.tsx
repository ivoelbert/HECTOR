import React from 'react';
import AceEditor from 'react-ace';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/mode-golang';

interface CodeEditorProps {
    code: string;
    setCode: React.Dispatch<React.SetStateAction<string>>;
}

export const CodeEditor: React.FC<CodeEditorProps> = props => {
    const { code, setCode } = props;

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
        />
    );
};
