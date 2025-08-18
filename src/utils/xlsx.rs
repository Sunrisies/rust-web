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
        "前水位(L)",
        "前水位(H)",
        "控制模式",
        "开启状态",
        "开度高度",
        "后水位(B)",
        "后水位(A)",
        "ICCID",
        "电池电压",
        "信号强度",
        "经度",
        "纬度",
        "时间",
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
        write_decimal(&mut worksheet, row, 1, &model.lvll)?;
        write_decimal(&mut worksheet, row, 2, &model.lvlh)?;
        let value = match model.ctrlmod {
            Some(0) => "手动",
            Some(1) => "闸前水位",
            Some(2) => "闸后水位",
            _ => "未知", // 如果 ctrlmod 的值不是 0, 1, 或 2，写入 "未知"
        };
        worksheet.write(row, 3, value)?;
        worksheet.write(row, 4, model.opensta)?;
        write_decimal(&mut worksheet, row, 5, &model.openhgt)?;
        write_decimal(&mut worksheet, row, 6, &model.lvlb)?;
        write_decimal(&mut worksheet, row, 7, &model.lvla)?;
        worksheet.write(row, 8, model.iccid.as_deref().unwrap_or(""))?;
        write_decimal(&mut worksheet, row, 9, &model.vbat)?;
        worksheet.write(row, 10, model.csq)?;
        write_decimal(&mut worksheet, row, 11, &model.lo)?;
        write_decimal(&mut worksheet, row, 12, &model.la)?;
        worksheet.write(row, 13, model.create_time.clone())?;
    }

    // 自动调整列宽
    worksheet.autofit();
    worksheet.set_column_width(0, 25.0)?;
    worksheet.set_column_width(1, 10.0)?;
    worksheet.set_column_width(2, 10.0)?;
    worksheet.set_column_width(3, 10.0)?;
    worksheet.set_column_width(4, 10.0)?;
    worksheet.set_column_width(5, 10.0)?;
    worksheet.set_column_width(6, 10.0)?;
    worksheet.set_column_width(7, 10.0)?;
    worksheet.set_column_width(9, 10.0)?;

    worksheet.set_column_width(8, 25.0)?;
    worksheet.set_column_width(10, 15.0)?;

    worksheet.set_column_width(13, 25.0)?;

    let buf = workbook.save_to_buffer()?;

    Ok(buf)
}
