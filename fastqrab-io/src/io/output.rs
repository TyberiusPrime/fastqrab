pub mod chunked_writer;
pub mod simulated_failure;

#[cfg(test)]
mod tests {
    use super::chunked_writer::{
        BamSinkOptions, ChunkPaths, ChunkPolicy, ChunkedRecordWriter, SinkConfig, WriteTarget,
    };
    use crate::blocks::{FastQChunk, OwnedFastQRead};
    use crate::io::reads::Tags;
    use fastqrab_config::FileFormat;
    use noodles::bam;
    use noodles::sam::alignment::record::Flags as SamFlags;
    use stringpod::CrossPods;
    use tempfile::TempDir;

    fn make_read_block() -> FastQChunk {
        let read = OwnedFastQRead {
            name: b"testread".into(),
            seq: b"ACGT".into(),
            qual: b"IIII".into(),
            plus: b"".into(),
        };
        FastQChunk::from_owned_reads(&[read])
    }

    fn write_and_read_flags(segment_index: usize, segment_count: usize) -> SamFlags {
        let dir = TempDir::new().unwrap();
        let target = WriteTarget::Files(ChunkPaths {
            directory: dir.path().to_path_buf(),
            basename: "out".to_string(),
            suffix: "bam".to_string(),
        });
        let bam_options = BamSinkOptions {
            comment_separation_char: b' ',
            ..Default::default()
        };
        let mut writer = ChunkedRecordWriter::new(
            FileFormat::Bam,
            target,
            SinkConfig::default(),
            ChunkPolicy::default(),
            Some(bam_options),
            std::num::NonZero::<usize>::MIN,
            true,
        )
        .unwrap();
        let block = make_read_block();
        let tags = Tags::default();
        writer
            .write_bam_record(&block.get(0), 0, segment_index, segment_count, &tags)
            .unwrap();
        let () = writer.finish().unwrap();

        let path = dir.path().join("out.bam");
        let mut reader = bam::io::Reader::new(std::fs::File::open(&path).unwrap());
        let _header = reader.read_header().unwrap();
        let mut record = bam::record::Record::default();
        let bytes_read = reader.read_record(&mut record).unwrap();
        assert_ne!(bytes_read, 0, "Expected a record in BAM output");
        record.flags()
    }

    #[test]
    fn test_bam_flags_single_read() {
        let flags = write_and_read_flags(0, 1);
        assert_eq!(flags, SamFlags::UNMAPPED);
    }

    #[test]
    fn test_bam_flags_first_of_two() {
        let flags = write_and_read_flags(0, 2);
        assert_eq!(
            flags,
            SamFlags::UNMAPPED
                | SamFlags::SEGMENTED
                | SamFlags::MATE_UNMAPPED
                | SamFlags::FIRST_SEGMENT
        );
    }

    #[test]
    fn test_bam_flags_last_of_two() {
        let flags = write_and_read_flags(1, 2);
        assert_eq!(
            flags,
            SamFlags::UNMAPPED
                | SamFlags::SEGMENTED
                | SamFlags::MATE_UNMAPPED
                | SamFlags::LAST_SEGMENT
        );
    }

    #[test]
    fn test_bam_flags_first_of_four() {
        let flags = write_and_read_flags(0, 4);
        assert_eq!(
            flags,
            SamFlags::UNMAPPED
                | SamFlags::SEGMENTED
                | SamFlags::MATE_UNMAPPED
                | SamFlags::FIRST_SEGMENT
        );
    }

    #[test]
    fn test_bam_flags_middle_of_four() {
        for idx in [1_usize, 2_usize] {
            let flags = write_and_read_flags(idx, 4);
            assert_eq!(
                flags,
                SamFlags::UNMAPPED | SamFlags::SEGMENTED | SamFlags::MATE_UNMAPPED,
                "Failed for segment_index={idx}"
            );
        }
    }

    #[test]
    fn test_bam_flags_last_of_four() {
        let flags = write_and_read_flags(3, 4);
        assert_eq!(
            flags,
            SamFlags::UNMAPPED
                | SamFlags::SEGMENTED
                | SamFlags::MATE_UNMAPPED
                | SamFlags::LAST_SEGMENT
        );
    }
}
