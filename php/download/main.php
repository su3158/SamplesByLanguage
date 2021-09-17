<?php

    $filedata = 'test';
    $mimeType = null;
    $filedata = !empty($filedata) ? $filedata : PHP_EOL;
    $temp = tmpfile();
    $logpath = stream_get_meta_data($temp)['uri'];
    fwrite($temp, $filedata);
    //-- Content-Typeとして送信するMIMEタイプ(第2引数を渡さない場合は自動判定) ※詳細は後述
    $mimeType = (isset($mimeType)) ? $mimeType
                                    : (new finfo(FILEINFO_MIME_TYPE))->file($logpath);
    //-- 適切なMIMEタイプが得られない時は、未知のファイルを示すapplication/octet-streamとする
    if (!preg_match('/\A\S+?\/\S+/', $mimeType)) {
        $mimeType = 'application/octet-stream';
    }

    header('Content-Type: ' . $mimeType);
    header('X-Content-Type-Options: nosniff');
    header('Content-Length: ' . filesize($logpath));
    header('Content-Disposition: attachment; filename="' . basename($logpath) . '.txt"');
    header('Connection: close');
    while (ob_get_level()) {
        ob_end_clean(); 
    }
    readfile($logpath);
    fclose($logpath);
    exit;