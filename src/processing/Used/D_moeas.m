addpath(".");

close all
clear
clc
format long g

root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "aggregate", "MOEA");
out_folder = fullfile(root_path, "processed", "MOEA");

if ~exist(out_folder, 'dir')
    mkdir(out_folder)
end

sizes = ["12", "16", "20", "24", "28", "32"];

algs = ["NSGAII", "IBEA", "MOEAD"];

for j = 1:length(algs)
    alg = algs(j);
    
    rows = [];
    for i = 1:length(sizes)
        size = sizes(i);
        
        hv_file = fullfile(src_folder, size, alg, 'aggregate.csv');
        
        agg = csvread(hv_file);
        
        median = agg(end, 6);
        
        lq = median - agg(end, 5);
        uq = agg(end, 7) - median;
        
        out_size = (str2double(size) ^ 3) / 4;
        rows(i, :) = [out_size; lq; median; uq];
    end
    
    out_file = fullfile(out_folder, append(alg, '.csv'));
    fid = fopen(out_file, 'w');
    
    csvwrite(out_file, rows);
end

fclose(fid);