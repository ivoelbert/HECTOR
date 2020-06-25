import React, { CSSProperties } from 'react';
import AceEditor, { IEditorProps } from 'react-ace';
import { useCtrlKeys } from '../../hooks/useCtrlKeys';
import { Examples } from './Examples';
import { CompileCodeAction } from '../../hooks/useCompileResult';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/mode-golang';
import './CodeEditor.scss';

interface CodeEditorProps {
    code: string;
    setCode: React.Dispatch<React.SetStateAction<string>>;
    compileCode: CompileCodeAction;
}

export const CodeEditor: React.FC<CodeEditorProps> = ({ compileCode, code, setCode }) => {
    useCtrlKeys([13, 83], () => compileCode(code));

    const editorStyles: CSSProperties = {
        height: '600px',
        fontSize: '14px',
        width: '85%',
    };

    return (
        <div className="code-editor-container">
            <Examples setCode={setCode} compileCode={compileCode} />
            <AceEditor
                mode="golang"
                theme="monokai"
                onChange={(newCode) => setCode(newCode)}
                value={code}
                name="code-editor"
                editorProps={{ $blockScrolling: true }}
                style={editorStyles}
                showPrintMargin={false}
                onLoad={(editor: IEditorProps) => editor.focus()}
                onBlur={() => compileCode(code)}
            />
        </div>
    );
};
