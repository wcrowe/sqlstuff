<Query Kind="Program">
  <Connection>
    <ID>4154a166-b104-4b78-87d0-5b3fcf57870e</ID>
    <NamingServiceVersion>2</NamingServiceVersion>
    <Persist>true</Persist>
    <Server>localhost</Server>
    <AllowDateOnlyTimeOnly>true</AllowDateOnlyTimeOnly>
    <SqlSecurity>true</SqlSecurity>
    <UserName>sa</UserName>
    <Password>AQAAANCMnd8BFdERjHoAwE/Cl+sBAAAAFeZGTaeVlU6uamO9+HeboQAAAAACAAAAAAAQZgAAAAEAACAAAABh5EQFVuJ/+QpxBh/ozcYtwq/yg30WoWym2ArKYuwphQAAAAAOgAAAAAIAACAAAAAhYmy05cS14Ud/pn2xKHdMAZW8+i1FUwfy/yn8sTFEnxAAAACVIi1CKC1RsR0uWwjA5pi2QAAAAKkEOfcQe8kBOpkzmg9klBRshtiVuSMQf+CMJzJTckG2GBtHzyemGlLVkce5bhH9Y8TFEqPEsjOvq5/1fXRXULc=</Password>
    <NoCapitalization>true</NoCapitalization>
    <Database>work</Database>
  </Connection>
</Query>

void Main()
{

	var buffers = GetPhoto(@"c:\dev\allbooks.xml");
//	var blob = new TestRow();

	var blob = TestRows.First(tr =>tr.Id == 1 );
//	blob.Dump();
//	blob.ImageRow = buffers;
// SubmitChanges();
//	Tbl_Blobs.InsertOnSubmit(blob);
//	SubmitChanges();

//	var readblob = Tbl_Blobs.FirstOrDefault(tb => tb.Blob_id == Guid.Parse("f803b2b6-2300-4295-82e4-e941a2cac0da")).Blob;
	var s = blob.ImageRow.ToArray();
	MemoryStream ms = new MemoryStream();
	ms.Write(s,0,s.Length);
	ms.Position = 0;


	string st = Encoding.UTF8.GetString(s);
	st.Dump();
}

void InsertRow(){
	var maxId = TestRows.Max(tr => tr.Id);
	var blob = new TestRow();
	blob.Id = maxId + 1;
	blob.VarCharRow = "$Hello Varchar row {blob.Id}";
	blob.NVarCharRow = "$Hello NVarchar row {blob.Id}";
	blob.UuidRow = Guid.NewGuid();
	blob.LongRow = DateTime.Now.Ticks;
	blob.DateTimeRow = DateTime.UtcNow;
	blob.SmallIntRow = Convert.ToInt16(DateTime.Now.Year + DateTime.Now.Month + DateTime.Now.Day );
	blob.BitRow = DateTime.Now.Minute %2 == 0;
	blob.FloatRow = float.MaxValue - Convert.ToSingle(3.41*blob.Id);
	blob.DoubleRow = double.MaxValue -  Convert.ToSingle(3.41*blob.Id);
	blob.RealRow = Convert.ToSingle(3.41*blob.Id);
	TestRows.InsertOnSubmit(blob);
	SubmitChanges();
}

public static byte[] GetPhoto(string filePath)
{
	FileStream stream = new FileStream(
		filePath, FileMode.Open, FileAccess.Read);
	BinaryReader reader = new BinaryReader(stream);

	byte[] photo = reader.ReadBytes((int)stream.Length);

	reader.Close();
	stream.Close();

	return photo;
}
//string filename = Path.GetFileName(@"c:\dev\allbooks.xml");
//var stream = new StreamReader(@"c:\dev\allbooks.xml");
//using (var fs = stream.ReadToEnd())
//{
//	using (BinaryReader br = new BinaryReader(stream))
//	{
//		byte[] bytes = br.ReadBytes((Int32)fs.Length);
//	
//		using (SqlConnection con = new this.)
//		{
//			string query = "insert into tblFiles values (@Name, @ContentType, @Data)";
//			using (SqlCommand cmd = new SqlCommand(query))
//			{
//				cmd.Connection = con;
//				cmd.Parameters.AddWithValue("@Name", filename);
//				cmd.Parameters.AddWithValue("@ContentType", contentType);
//				cmd.Parameters.AddWithValue("@Data", bytes);
//				con.Open();
//				cmd.ExecuteNonQuery();
//				con.Close();
//			}
//		}
//	}
//}
