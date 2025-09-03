use crate::models::sluice_data::{self};
use num_traits::ToPrimitive;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError, *};
use sea_orm::prelude::Decimal;
pub fn generate_excel(models: &[sluice_data::SluiceWithUsername]) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let mut worksheet = workbook.add_worksheet();

    // 设置标题格式
    let header_format = Format::new().set_bold().set_align(FormatAlign::Center);

    // 写入表头
    let headers = [
        "闸站名称",
        "水位高度",
        "开启状态",
        "闸门高度",
        "电池电压",
        "信号强度",
        "更新时间",
        "位置坐标",
    ];

    for (col, header) in headers.iter().enumerate() {
        worksheet.write_with_format(0, col as u16, *header, &header_format)?;
    }

    // 写入数据行
    for (row_idx, model) in models.iter().enumerate() {
        let row = row_idx as u32 + 1;

        // 辅助函数：处理 Decimal 类型
        fn write_decimal(
            worksheet: &mut Worksheet,
            row: u32,
            col: u16,
            value: &Option<Decimal>,
        ) -> Result<(), XlsxError> {
            match value {
                Some(d) => worksheet.write(row, col, d.to_f64()),
                None => {
                    // 创建默认格式
                    let default_format = rust_xlsxwriter::Format::default();
                    worksheet.write_blank(row, col, &default_format)
                }
            }?;
            Ok(())
        }

        // 按列顺序写入数据
        worksheet.write(row, 0, &model.sluice_name)?;
        write_decimal(&mut worksheet, row, 1, &model.lvla)?;
        let value = match model.opensta {
            Some(0) => "关闸",
            Some(1) => "开闸",
            Some(2) => "停止",
            _ => "未知",
        };
        worksheet.write(row, 2, value)?;
        write_decimal(&mut worksheet, row, 3, &model.openhgt)?;
        write_decimal(&mut worksheet, row, 4, &model.vbat)?;
        let csq = match model.csq {
            Some(csq) => format!("{:?} dBm", csq),
            None => "".to_string(),
        };
        worksheet.write(row, 5, csq)?;
        worksheet.write(row, 6, model.create_time.clone())?;
        let lon_and_lat = if let (Some(lo), Some(la)) = (&model.lo, &model.la) {
            format!("{}, {}", lo, la)
        } else {
            "".to_string()
        };
        worksheet.write(row, 7, lon_and_lat)?;
    }

    // 自动调整列宽
    worksheet.autofit();
    worksheet.set_column_width(0, 25.0)?;
    worksheet.set_column_width(1, 10.0)?;
    worksheet.set_column_width(2, 10.0)?;
    worksheet.set_column_width(3, 10.0)?;
    worksheet.set_column_width(4, 10.0)?;
    worksheet.set_column_width(5, 10.0)?;
    worksheet.set_column_width(6, 20.0)?;
    worksheet.set_column_width(7, 20.0)?;

    let buf = workbook.save_to_buffer()?;

    Ok(buf)
}
