export interface KelasType {
  id: string;
  namaKelas: string;
  waliKelas: string;
  tingkat: string;
  tahunAjaran: string;
  createdAt?: Date;
  updatedAt?: Date;
}

export interface KelasFormData {
  namaKelas: string;
  waliKelas: string;
  tingkat: string;
  tahunAjaran: string;
}
