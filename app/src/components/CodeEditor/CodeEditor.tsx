import React, { CSSProperties } from 'react';
import AceEditor, { IEditorProps } from 'react-ace';
import { useLocalStorageState } from '../../hooks/useLocalStorageState';
import { useCtrlKeys } from '../../hooks/useCtrlKeys';
import { baseCode } from '../../utils/baseCode';
import { Examples } from './Examples';
import 'ace-builds/src-noconflict/theme-monokai';
import 'ace-builds/src-noconflict/mode-golang';
import './CodeEditor.scss';

interface CodeEditorProps {
    compileCode: (code: string) => void;
}

export const CodeEditor: React.FC<CodeEditorProps> = (props) => {
    const { compileCode } = props;

    const [code, setCode] = useLocalStorageState<string>('hector-code', baseCode);

    useCtrlKeys([13, 83], () => compileCode(code));

    const editorStyles: CSSProperties = {
        height: '600px',
        fontSize: '14px',
        width: '85%',
    };

    return (
        <div className="code-editor-container">
            <Examples setCode={setCode} />
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
