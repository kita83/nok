use rodio::{OutputStream, Sink, source::{SineWave, Source}};
use std::time::Duration;

pub fn play_knock_sound() -> Result<(), String> {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| format!("Failed to get output stream: {}", e))?;
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| format!("Failed to create sink: {}", e))?;

    // ノック音を模倣した短いビープ音を3回再生
    for i in 0..3 {
        // 200Hzの短いビープ音（0.1秒）
        let source = SineWave::new(200.0)
            .take_duration(Duration::from_millis(100))
            .amplify(0.3); // 音量を30%に設定

        sink.append(source);

        // ノックの間に50msの間隔を追加（最後のノック以外）
        if i < 2 {
            let silence = SineWave::new(0.0)
                .take_duration(Duration::from_millis(50))
                .amplify(0.0);
            sink.append(silence);
        }
    }

    sink.sleep_until_end(); // すべての音が再生されるまで待機
    Ok(())
}
